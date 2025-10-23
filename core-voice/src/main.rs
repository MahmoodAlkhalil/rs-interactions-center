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

    // let nats_topic = env::var("NATS_VOICE_TOPIC");
    // let nats_topic = match nats_topic {
    //     Ok(data) => data,
    //     Err(_) => {
    //         return Err(IcError {
    //             status_code: StatusCode::INTERNAL_SERVER_ERROR,
    //             message: "NATS_VOICE_TOPIC env variable is not set".to_string(),
    //         });
    //     }
    // };
    // match Uuid::from_str(&nats_topic) {
    //     Ok(_) => {}
    //     Err(_) => {
    //         return Err(IcError {
    //             status_code: StatusCode::INTERNAL_SERVER_ERROR,
    //             message: "NATS_VOICE_TOPIC env variable is not a UUID".to_string(),
    //         });
    //     }
    // }
    // println!(
    //     "NATS voice channel topic name {} ",
    //     env::var("NATS_VOICE_TOPIC").unwrap()
    // );
    // let nats_client = match init_nats_client().await {
    //     Ok(data) => data,
    //     Err(error) => {
    //         error!("NATS error {}", error);
    //         return Err(IcError {
    //             status_code: StatusCode::INTERNAL_SERVER_ERROR,
    //             message: "faild to create nats client".to_string(),
    //         });
    //     }
    // };
    // let voice_topic_sub = nats_client
    //     .subscribe(format!("channel.{}", nats_topic))
    //     .await;
    // let mut voice_topic_sub = match voice_topic_sub {
    //     Ok(data) => data,
    //     Err(error) => {
    //         error!("NATS error {}", error);
    //         return Err(IcError {
    //             status_code: StatusCode::INTERNAL_SERVER_ERROR,
    //             message: "faild to subscribe to voice channel topic".to_string(),
    //         });
    //     }
    // };
    // while true {
    //     let data = voice_topic_sub.next().await;
    // }
    Ok(())
}
