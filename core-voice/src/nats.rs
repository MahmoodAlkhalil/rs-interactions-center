use async_nats::Client;
use core_engine_dto::errors::ApiError;
use nkeys::KeyPair;
use std::env;

pub async fn create_client() -> Result<Client, ApiError> {
    let url = env::var("NATS_URL").unwrap();
    let jwt = env::var("NATS_JWT").unwrap();
    let seed = env::var("NATS_SEED").unwrap();
    let key_pair = KeyPair::from_seed(&seed)?;
    let options: async_nats::ConnectOptions =
        async_nats::ConnectOptions::new().jwt(jwt, move |nonce| {
            let key_pair = key_pair.clone();
            async move { key_pair.sign(&nonce).map_err(async_nats::AuthError::new) }
        });
    let nats_client = async_nats::connect_with_options(url, options).await?;
    Ok(nats_client)
}
