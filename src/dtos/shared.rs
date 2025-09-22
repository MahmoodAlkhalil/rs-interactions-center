use sea_orm::{ConnectionTrait, TransactionTrait};

pub struct ServiceDto<'a, A, B>
where
    B: ConnectionTrait + TransactionTrait,
{
    pub request: A,
    pub db: &'a B,
}
