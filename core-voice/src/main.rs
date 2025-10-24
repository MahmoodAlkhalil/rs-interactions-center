use core_engine_dto::errors::IcError;
use tracing::Level;

use crate::processes::{local_db::init_kv_local_db, nats::init_nats_client};
mod processes;
#[tokio::main]
async fn main() -> Result<(), IcError> {
    dotenv::from_filename(".env.core-voice").ok();
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    init_kv_local_db().await?;
    init_nats_client().await?;
    Ok(())
}
