use crate::db::cluster_locks::tx_lock;
use crate::db::entities::channels::{Column as ChannelsC, Entity as ChannelsE};
use crate::db::entities::interactions::Entity as InteractionsE;
use crate::db::entities::interactions_events::{
    ActiveModel as InteractionsEventsAM, Entity as InteractionsEventsE,
};
use crate::db::entities::queues::{ActiveModel as QueuesAM, Entity as QueuesE};
use crate::db::entities::queues_channels_assignment::{
    ActiveModel as QueuesChannelsAssignmentAM, Column as QueuesChannelsAssignmentC,
    Entity as QueuesChannelsAssignmentE,
};
use crate::db::entities::runtime_interactions_queues::{
    ActiveModel as RuntimeInteractionsQueuesAM, Column as RuntimeInteractionsQueuesC,
    Entity as RuntimeInteractionsQueuesE,
};
use crate::db::links::queues::QueuesToChannels;
use crate::dtos::queues::requests::{
    AssignChannelsToQueue, CreateQueue, DequeueInteraction, EnqueueInteraction,
};
use crate::dtos::queues::responses::{
    Queue as QueueRes, QueuesChannelsAssignment as QueuesChannelsAssignmentResponse,
};
use crate::dtos::shared::RequestDto;
use crate::utils::errors::{IcError, NoType};
use crate::utils::validators::validate_interaction_state_change;
use axum::http::StatusCode;
use core_engine_consts::defaults::DEFAULT_QUEUE_PRIORITY;
use core_engine_consts::interaction_states::InteractionStates;
use sea_orm::prelude::*;
use sea_orm::{ConnectionTrait, IntoActiveModel, Set, TransactionSession, TransactionTrait};

pub async fn get_all<B>(request: &RequestDto<'_, NoType, B>) -> Result<Vec<QueueRes>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let queues = QueuesE::find()
        .find_with_linked(QueuesToChannels)
        .all(request.db)
        .await?;
    Ok(queues.iter().map(|q| q.into()).collect())
}

pub async fn create<B>(request: &RequestDto<'_, CreateQueue, B>) -> Result<QueueRes, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let tx = request.db.begin().await?;
    let queue = QueuesAM {
        id: Set(Uuid::now_v7()),
        name: Set(request.data.as_ref().unwrap().name.clone()),
        ..Default::default()
    };
    let queue = QueuesE::insert(queue).exec_with_returning(&tx).await?;
    let channels = ChannelsE::find()
        .filter(ChannelsC::Id.is_in(request.data.as_ref().unwrap().channels.iter().copied()))
        .all(&tx)
        .await?;
    if channels.len() != request.data.as_ref().unwrap().channels.len() {
        return Err(IcError {
            status_code: StatusCode::BAD_REQUEST,
            message: "Some or all channels are not found".to_string(),
        });
    }
    QueuesChannelsAssignmentE::insert_many(channels.iter().map(|channel| {
        QueuesChannelsAssignmentAM {
            queue_id: Set(queue.id),
            channel_id: Set(channel.id),
            ..Default::default()
        }
    }))
    .exec(&tx)
    .await?;
    Ok(queue.into())
}

pub async fn replace_queues_channels_assignments<B>(
    request: RequestDto<'_, AssignChannelsToQueue, B>,
) -> Result<QueuesChannelsAssignmentResponse, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let tx = request.db.begin().await?;
    let queue = QueuesE::find_by_id(request.data.as_ref().unwrap().id)
        .one(request.db)
        .await?
        .ok_or(IcError {
            status_code: StatusCode::BAD_REQUEST,
            message: "queue not found".to_string(),
        })?;
    let channels = ChannelsE::find()
        .filter(ChannelsC::Id.is_in(request.data.as_ref().unwrap().channels.clone()))
        .all(request.db)
        .await?;
    if channels.len() != request.data.as_ref().unwrap().channels.len() {
        return Err(IcError {
            status_code: StatusCode::BAD_REQUEST,
            message: "some or all channels are not found".to_string(),
        });
    }
    QueuesChannelsAssignmentE::delete_many()
        .filter(QueuesChannelsAssignmentC::QueueId.eq(request.data.as_ref().unwrap().id))
        .exec(request.db)
        .await?;
    let new_assignments =
        QueuesChannelsAssignmentE::insert_many(request.data.as_ref().unwrap().channels.iter().map(
            |channel| QueuesChannelsAssignmentAM {
                queue_id: Set(request.data.as_ref().unwrap().id),
                channel_id: Set(channel.clone()),
                ..Default::default()
            },
        ))
        .exec_with_returning(request.db)
        .await?;
    tx.commit().await?;
    Ok(QueuesChannelsAssignmentResponse {
        id: queue.id,
        channels: new_assignments.iter().map(|item| item.channel_id).collect(),
    })
}

pub async fn enqueue_interaction<B>(
    request: &RequestDto<'_, EnqueueInteraction, B>,
) -> Result<(), IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let tx = request.db.begin().await?;
    tx_lock(request.data.as_ref().unwrap().interaction_id, &tx).await?;
    tx_lock(request.data.as_ref().unwrap().queue_id, &tx).await?;
    let queue = QueuesE::find_by_id(request.data.as_ref().unwrap().queue_id)
        .one(request.db)
        .await?
        .ok_or(IcError {
            status_code: StatusCode::BAD_REQUEST,
            message: "queue not found".to_string(),
        })?;
    let interaction = InteractionsE::find_by_id(request.data.as_ref().unwrap().interaction_id)
        .one(request.db)
        .await?
        .ok_or(IcError {
            status_code: StatusCode::BAD_REQUEST,
            message: "interaction not found".to_string(),
        })?;
    let new_state = InteractionStates::Enqueued;
    validate_interaction_state_change(InteractionStates::try_from(interaction.state)?, new_state)?;
    let enqueued_interaction = RuntimeInteractionsQueuesAM {
        queue_id: Set(queue.id),
        interaction_id: Set(interaction.id),
        priority: Set(request
            .data
            .as_ref()
            .unwrap()
            .priority
            .unwrap_or(DEFAULT_QUEUE_PRIORITY)),
        ..Default::default()
    };
    RuntimeInteractionsQueuesE::insert(enqueued_interaction)
        .exec_without_returning(&tx)
        .await?;
    let interaction_event = InteractionsEventsAM {
        interaction_id: Set(interaction.id),
        old_state: Set(interaction.state),
        new_state: Set(new_state.into()),
        queue_id: Set(Some(queue.id)),
        ..Default::default()
    };
    InteractionsEventsE::insert(interaction_event)
        .exec_without_returning(&tx)
        .await?;
    let mut interaction = interaction.into_active_model();
    interaction.state = Set(new_state.into());
    InteractionsE::update(interaction).exec(&tx).await?;
    tx.commit().await?;
    Ok(())
}

pub async fn dequeue_interaction<B>(
    request: &RequestDto<'_, DequeueInteraction, B>,
) -> Result<(), IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let tx = request.db.begin().await?;
    tx_lock(request.data.as_ref().unwrap().interaction_id, &tx).await?;
    let enqueued_interaction = RuntimeInteractionsQueuesE::find()
        .filter(
            RuntimeInteractionsQueuesC::InteractionId.eq(request
                .data
                .as_ref()
                .unwrap()
                .interaction_id),
        )
        .one(&tx)
        .await?
        .ok_or(IcError {
            status_code: StatusCode::BAD_REQUEST,
            message: "Interaction is not in a queue".to_string(),
        })?;
    tx_lock(enqueued_interaction.queue_id, &tx).await?;
    let interaction = InteractionsE::find_by_id(request.data.as_ref().unwrap().interaction_id)
        .one(&tx)
        .await?
        .ok_or(IcError {
            status_code: StatusCode::BAD_REQUEST,
            message: "interaction not found".to_string(),
        })?;
    let new_state = InteractionStates::Dequeued;
    validate_interaction_state_change(InteractionStates::try_from(interaction.state)?, new_state)?;
    RuntimeInteractionsQueuesE::delete_by_id((
        enqueued_interaction.queue_id,
        enqueued_interaction.interaction_id,
    ))
    .exec(&tx)
    .await?;
    let interaction_event = InteractionsEventsAM {
        interaction_id: Set(interaction.id),
        old_state: Set(interaction.state),
        new_state: Set(new_state.into()),
        queue_id: Set(Some(enqueued_interaction.queue_id)),
        ..Default::default()
    };
    InteractionsEventsE::insert(interaction_event)
        .exec_without_returning(&tx)
        .await?;
    let mut interaction = interaction.into_active_model();
    interaction.state = Set(new_state.into());
    InteractionsE::update(interaction).exec(&tx).await?;
    tx.commit().await?;
    Ok(())
}
