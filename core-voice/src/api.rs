use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock},
    time::Duration,
};

use async_nats::client;
use axum::{
    http::{HeaderMap, HeaderValue},
    routing::head,
};
use core_engine_dto::{Channel, KeycloakTokenResponse, errors::ApiError};
use reqwest::{Client, header};
use serde_json::json;
use tracing::info;

pub struct ApiClient {
    client: Client,
    token: KeycloakTokenResponse,
    core_engine_base_url: String,
    client_id: String,
}

impl ApiClient {
    pub async fn new() -> Result<Self, ApiError> {
        let token = generate_token().await?;
        let client = create_client(&token.access_token).await?;
        Ok(ApiClient {
            client,
            token,
            core_engine_base_url: std::env::var("CORE_ENGINE_BASE_URL").unwrap(),
            client_id: std::env::var("SSO_CLIENT_ID").unwrap(),
        })
    }

    pub async fn get_channel(self: &Self) -> Result<Channel, ApiError> {
        let channel: Channel = self
            .client
            .get(format!(
                "{}/api/v1/channels/{}",
                self.core_engine_base_url, self.client_id
            ))
            .send()
            .await?
            .json()
            .await?;
        Ok(channel)
    }
}

async fn create_client(token: &String) -> Result<Client, ApiError> {
    info!("creating API Client");
    let bearer_token = format!("Bearer {}", *token);
    let mut headers = HeaderMap::new();

    let mut auth_value = header::HeaderValue::from_str(bearer_token.as_str())?;
    auth_value.set_sensitive(true);
    headers.insert(header::AUTHORIZATION, auth_value);

    let client = Client::builder()
        .connect_timeout(Duration::from_secs(30))
        .default_headers(headers)
        .build()?;
    Ok(client)
}

async fn generate_token() -> Result<KeycloakTokenResponse, ApiError> {
    info!("requesting token from keycloak");
    let request_url = format!(
        "{}/realms/{}/protocol/openid-connect/token",
        std::env::var("SSO_BASE_URL")?,
        std::env::var("SSO_REALM")?
    );
    info!("client_id [{}]", std::env::var("SSO_CLIENT_ID")?);
    info!("request url [{}]", request_url);
    let client_id = std::env::var("SSO_CLIENT_ID")?;
    let client_secret = std::env::var("SSO_CLIENT_SECRET")?;

    let mut params = HashMap::new();
    params.insert("grant_type", "client_credentials");
    params.insert("client_id", client_id.as_str());
    params.insert("client_secret", client_secret.as_str());
    let response = Client::new()
        .post(request_url)
        .form(&params)
        .send()
        .await?
        .error_for_status()?;

    let token: KeycloakTokenResponse =
        serde_json::from_str(&response.text_with_charset("urf-8").await?)?;
    Ok(token)
}
