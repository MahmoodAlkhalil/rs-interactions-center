use crate::entities::channels::Entity as ChannelsE;
use crate::entities::groups::Entity as GroupsE;
use crate::entities::queues::Entity as QueuesE;
use crate::entities::queues_channels_assignment::Relation as QueuesChannelsAssignmentR;
use crate::entities::queues_groups_assignment::Relation as QueuesGroupsAssignmentR;
use sea_orm::{Linked, RelationDef, RelationTrait};

pub struct QueuesToChannels;
impl Linked for QueuesToChannels {
    type FromEntity = QueuesE;
    type ToEntity = ChannelsE;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            QueuesChannelsAssignmentR::Queues.def().rev(),
            QueuesChannelsAssignmentR::Channels.def(),
        ]
    }
}

pub struct QueuesToGroups;
impl Linked for QueuesToGroups {
    type FromEntity = QueuesE;
    type ToEntity = GroupsE;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            QueuesGroupsAssignmentR::Queues.def().rev(),
            QueuesGroupsAssignmentR::Groups.def(),
        ]
    }
}
