use crate::entities::groups::Entity as GroupsE;
use crate::entities::users::Entity as UsersE;
use crate::entities::users_groups_assignment::Relation as UsersGroupsAssignmentR;
use sea_orm::{Linked, RelationDef, RelationTrait};

pub struct UsersToGroups;
impl Linked for UsersToGroups {
    type FromEntity = UsersE;
    type ToEntity = GroupsE;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            UsersGroupsAssignmentR::Users.def().rev(),
            UsersGroupsAssignmentR::Groups.def(),
        ]
    }
}
