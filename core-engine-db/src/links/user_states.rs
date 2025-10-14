use crate::entities::user_states::Entity;
use crate::entities::user_states::Relation;
use sea_orm::{Linked, RelationDef, RelationTrait};
pub struct UserStateSelfLink;

impl Linked for UserStateSelfLink {
    type FromEntity = Entity;
    type ToEntity = Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Relation::SelfRef.def().rev()]
    }
}
