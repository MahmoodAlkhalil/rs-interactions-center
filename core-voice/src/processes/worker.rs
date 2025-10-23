use core_engine_const::channel_worker_states::ChannelWorkerStates;
use tokio::sync::{OnceCell, RwLock};

use crate::processes::{local_db::init_kv_local_db, nats::init_nats_client};

pub static WORKER_STATE: OnceCell<RwLock<ChannelWorkerStates>> = OnceCell::const_new();

pub async fn start() {
    WORKER_STATE.set(RwLock::new(ChannelWorkerStates::Degraded));
    init_kv_local_db().await;
    init_nats_client().await;
    
}
