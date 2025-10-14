use sea_orm::DatabaseBackend::Postgres;
use sea_orm::{ConnectionTrait, DbErr, ExecResult, Statement, TransactionTrait};
use tracing::{info, trace};
use uuid::Uuid;

fn hasher(id: &Uuid) -> u64 {
    let mut sum: u64 = 0;
    for slice in id.into_bytes() {
        sum += slice as u64;
    }
    sum
}

pub async fn tx_lock<B>(id: Uuid, db: &B) -> Result<ExecResult, DbErr>
where
    B: TransactionTrait + ConnectionTrait,
{
    let lock_id = hasher(&id);
    trace!("tx locking uuid [{}] with hash [{}]", id, lock_id);
    db.execute_raw(Statement::from_sql_and_values(
        Postgres,
        "SELECT pg_advisory_xact_lock($1)",
        [lock_id.into()],
    ))
    .await
}

pub async fn lock<B>(id: Uuid, db: &B) -> Result<ExecResult, DbErr>
where
    B: TransactionTrait + ConnectionTrait,
{
    let lock_id = hasher(&id);
    trace!("locking uuid [{}] with hash [{}]", id, lock_id);
    db.execute_raw(Statement::from_sql_and_values(
        Postgres,
        "SELECT pg_advisory_lock($1)",
        [lock_id.into()],
    ))
    .await
}

pub async fn unlock<B>(id: Uuid, db: B) -> Result<ExecResult, DbErr>
where
    B: TransactionTrait + ConnectionTrait,
{
    let lock_id = hasher(&id);
    trace!("unlocking uuid [{}] with hash [{}]", id, lock_id);
    db.execute_raw(Statement::from_sql_and_values(
        Postgres,
        "SELECT pg_advisory_unlock($1)",
        [lock_id.into()],
    ))
    .await
}
