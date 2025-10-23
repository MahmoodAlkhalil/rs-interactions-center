use core_engine_dto::errors::IcError;
use reqwest::StatusCode;
use sled::Db;
use tokio::sync::OnceCell;

static KV_LOCAL_STORE: OnceCell<Db> = OnceCell::const_new();

pub async fn init_kv_local_db() -> Result<(), IcError> {
    if KV_LOCAL_STORE.initialized() {
        return Err(IcError {
            status_code: StatusCode::BAD_REQUEST,
            message: "KV DB is already initialized".to_owned(),
        });
    }
    let path = std::env::var("LOCAL_DB_LOCATION")?;
    let seld_db = sled::open(path).unwrap();
    KV_LOCAL_STORE.set(seld_db)?;
    Ok(())
}

pub fn was_recovered() -> bool {
    KV_LOCAL_STORE.get().unwrap().was_recovered()
}

pub fn get_worker_id() -> Result<String, IcError> {
    let bytes = KV_LOCAL_STORE
        .get()
        .unwrap()
        .get("worker_id")?
        .ok_or(IcError::not_found("key not found"))?;
    Ok(String::from_utf8(bytes.to_vec()).unwrap())
}

pub fn set_worker_id(worker_id: &str) -> Result<(), IcError> {
    if get_worker_id().is_ok() {
        return Err(IcError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "worker_id is already sat, cannot set it again".to_owned(),
        });
    }
    KV_LOCAL_STORE
        .get()
        .unwrap()
        .insert("worker_id", worker_id)?;
    Ok(())
}
