use crate::shared::IcError;
use log::error;
use sea_orm::DbErr;
use uuid::Uuid;

pub fn extract<A>(id: Uuid, result: Result<A, DbErr>) -> Result<A, IcError> {
    match result {
        Ok(data) => Ok(data),
        Err(e) => {
            error!("[{}] database error [{}]", id, e.to_string());
            Err(IcError {
                id,
                message: "database error".to_string(),
            })
        }
    }
}
