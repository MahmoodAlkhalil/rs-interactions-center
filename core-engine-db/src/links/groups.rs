use crate::entities::groups::Entity as GroupsE;
use crate::entities::queues::Entity as QueuesE;
use crate::entities::queues_groups_assignment::Relation as QueuesGroupsAssignmentR;
use crate::entities::skills::Entity as SkillsE;
use crate::entities::skills_groups_assignment::Relation as SkillsToGroupsAssignmentR;
use crate::entities::users::Entity as UsersE;
use crate::entities::users_groups_assignment::Relation as UsersGroupsAssignmentR;

use sea_orm::{Linked, RelationDef, RelationTrait};

pub struct GroupsToUsers;
impl Linked for GroupsToUsers {
    type FromEntity = GroupsE;
    type ToEntity = UsersE;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            UsersGroupsAssignmentR::Groups.def().rev(),
            UsersGroupsAssignmentR::Users.def(),
        ]
    }
}

pub struct GroupsToQueues;
impl Linked for GroupsToQueues {
    type FromEntity = GroupsE;
    type ToEntity = QueuesE;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            QueuesGroupsAssignmentR::Groups.def().rev(),
            QueuesGroupsAssignmentR::Queues.def(),
        ]
    }
}

pub struct GroupsToSkills;
impl Linked for GroupsToSkills {
    type FromEntity = GroupsE;
    type ToEntity = SkillsE;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            SkillsToGroupsAssignmentR::Groups.def().rev(),
            SkillsToGroupsAssignmentR::Skills.def(),
        ]
    }
}
