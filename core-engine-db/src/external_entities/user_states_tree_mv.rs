use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_states_tree_mv")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "Text", unique)]
    pub name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub mark_for_delete: bool,
    pub system_state: bool,
    pub user_control_allowed: bool,
    pub created_at: DateTimeWithTimeZone,
    pub level: i32,
    pub path: Vec<Uuid>,
    #[sea_orm(column_type = "Text")]
    pub full_path: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
