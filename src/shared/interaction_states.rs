use sea_orm::entity::prelude::*;

// Using the derive macro
#[derive(Clone, Debug, PartialEq, EnumIter, DeriveActiveEnum, Eq)]
#[sea_orm(rs_type = "i32", db_type = "Integer", enum_name = "id")]
pub enum InteractionStatesEnum {
    #[sea_orm(num_value = 0)]
    New,
    #[sea_orm(num_value = 1000)]
    Enqueued,
    #[sea_orm(num_value = 2000)]
    Dequeued,
    #[sea_orm(num_value = 3000)]
    Active,
    #[sea_orm(num_value = 4000)]
    Disconnected,
    #[sea_orm(num_value = 5000)]
    Closed,
}
