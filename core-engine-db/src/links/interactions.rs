
use crate::entities::channels::Entity as ChannelsE;
use crate::entities::interactions::Entity as InteractionsE;
use crate::entities::queues::Entity as QueuesE;
use crate::entities::queues_channels_assignment::Relation as QueuesChannelsAssignmentR;
use crate::entities::queues_groups_assignment::Relation as QueuesGroupsAssignmentR;
use sea_orm::{Linked, RelationDef, RelationTrait};

pub struct InteractionsToChannels;
impl Linked for InteractionsToChannels {
    type FromEntity = InteractionsE;
    type ToEntity = ChannelsE;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            QueuesChannelsAssignmentR::Queues.def().rev(),
            QueuesChannelsAssignmentR::Channels.def(),
        ]
    }
}
