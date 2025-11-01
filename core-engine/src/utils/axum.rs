use async_nats::Client;
use axum::{Json, response::Response};
use axum::{extract::FromRef, http::request::Parts};
use axum::{extract::FromRequestParts, http::StatusCode, response::IntoResponse};
use axum_jwks::{Jwks, ParseTokenClaims, TokenError};
use futures_util::future::{BoxFuture, Map};
use sea_orm::DatabaseConnection;
use sea_orm::prelude::async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::task::{Context, Poll};
use std::{collections::HashMap, sync::Arc};
use tower::{Layer, Service};
use tracing::{Level, span};
use uuid::Uuid;

#[derive(Clone)]
pub struct SharedState {
    pub db_pool: Arc<DatabaseConnection>,
    pub nats_client: Arc<Client>,
    pub jwks: Jwks,
}

impl SharedState {
    pub fn new(db_pool: Arc<DatabaseConnection>, nats_client: Arc<Client>, jwks: Jwks) -> Self {
        SharedState {
            db_pool,
            nats_client,
            jwks,
        }
    }
}

pub struct InnerRequest<T> {
    pub id: Uuid,
    pub data: Option<T>,
    pub shared_state: SharedState,
    pub claims: TokenClaims,
}

impl<T> InnerRequest<T> {
    pub fn new(id: Uuid, data: Option<T>, shared_state: SharedState, claims: TokenClaims) -> Self {
        Self {
            id,
            data,
            shared_state,
            claims,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RequestId(pub Uuid);

impl RequestId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Clone)]
pub struct RequestIdLayer;

impl<S> Layer<S> for RequestIdLayer {
    type Service = RequestIdMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestIdMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct RequestIdMiddleware<S> {
    inner: S,
}

impl<S> Service<axum::extract::Request> for RequestIdMiddleware<S>
where
    S: Service<axum::extract::Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: axum::http::Request<axum::body::Body>) -> Self::Future {
        let id = Uuid::new_v4();
        let method = request.method().clone();
        let path = request.uri().path().to_owned();
        request.extensions_mut().insert(RequestId(id));

        let span = span!(
            Level::INFO,
            "request",
            %id,
            %method,
            %path
        );
        let future = self.inner.call(request);

        Box::pin(async move {
            let _enter = span.enter();
            let response: Response = future.await?;
            let mut response = response;
            response
                .headers_mut()
                .insert("X-Request-Id", id.to_string().parse().unwrap());
            Ok(response)
        })
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for RequestId
where
    S: Send + Sync,
{
    type Rejection = ();

    fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        let result = match parts.extensions.get::<RequestId>().cloned() {
            Some(request_id) => Ok(request_id),
            None => Err(()),
        };
        futures_util::future::ready(result)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TokenClaims {
    pub sub: String,
    pub realm_access: RealmAccess,
    pub client_id: Option<String>,
    pub preferred_username: Option<String>,
    pub aud: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RealmAccess {
    pub roles: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ResourcesAccess {
    pub roles: HashMap<String, Roles>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Roles {
    pub roles: Vec<String>,
}

impl ParseTokenClaims for TokenClaims {
    type Rejection = TokenClaimsError;
}

impl IntoResponse for TokenClaimsError {
    fn into_response(self) -> Response {
        let body = match self {
            Self::Invalid => json!({ "message": "Invalid token." }),
            Self::Missing => json!({ "message": "No token provided." }),
        };

        (StatusCode::UNAUTHORIZED, Json(body)).into_response()
    }
}
pub enum TokenClaimsError {
    Missing,
    Invalid,
}

impl From<TokenError> for TokenClaimsError {
    fn from(value: TokenError) -> Self {
        match value {
            TokenError::Missing => Self::Missing,
            other => Self::Invalid,
        }
    }
}

impl FromRef<SharedState> for Jwks {
    fn from_ref(state: &SharedState) -> Self {
        state.jwks.clone()
    }
}
