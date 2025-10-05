use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::{body::Body, extract::Request, response::Response};
use futures_util::future::BoxFuture;
use sea_orm::prelude::async_trait::async_trait;
use std::task::{Context, Poll};
use tower::{Layer, Service};
use tracing::{span, Level};
use uuid::Uuid;

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

impl<S> Service<Request> for RequestIdMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request) -> Self::Future {
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
