use axum::http::StatusCode;
use core_engine_const::defaults::DEFAULT_QUEUE_PRIORITY;
use core_engine_const::interaction_states::InteractionStates;
use core_engine_db::entities::channels::{Column as ChannelsC, Entity as ChannelsE};
use core_engine_db::entities::interactions::Entity as InteractionsE;
use core_engine_db::entities::interactions_events::{
    ActiveModel as InteractionsEventsAM, Entity as InteractionsEventsE,
};
use core_engine_db::entities::queues::{ActiveModel as QueuesAM, Entity as QueuesE};
use core_engine_db::entities::queues_channels_assignment::{
    ActiveModel as QueuesChannelsAssignmentAM, Column as QueuesChannelsAssignmentC,
    Entity as QueuesChannelsAssignmentE,
};
use core_engine_db::entities::runtime_interactions_queues::{
    ActiveModel as RuntimeInteractionsQueuesAM, Column as RuntimeInteractionsQueuesC,
    Entity as RuntimeInteractionsQueuesE,
};
use core_engine_db::links::queues::QueuesToChannels;
use core_engine_db::{cluster_locks::tx_lock, entities::channels};
use core_engine_dto::{
    ChannelMessage, ChannelMessageTarget, ChannelMessageType, Interaction, Queue, errors::ApiError,
};
use std::{
    io::Read,
    str::{Bytes, FromStr},
    thread::spawn,
};
use tracing::info;

use sea_orm::{ConnectionTrait, IntoActiveModel, Set, TransactionTrait};
use sea_orm::{TransactionSession, prelude::*};
use strum::EnumProperty;
use tower::util::error::optional::None;

use crate::{
    processes::queues::notify_channel_enqueued_interaction,
    utils::{
        axum::InnerRequest,
        conversions::{interaction_state_to_uuid, uuid_to_interaction_state},
        validators::validate_interaction_state_change,
    },
};

pub async fn get_all(request: InnerRequest<None>) -> Result<Vec<Queue>, ApiError> {
    let InnerRequest { claims, .. } = request;
    let queues = QueuesE::find()
        .find_with_linked(QueuesToChannels)
        .all(request.shared_state.db_pool.as_ref())
        .await?;
    Ok(queues.into_iter().map(|q| q.into()).collect())
}

pub async fn create(request: InnerRequest<Queue>) -> Result<Queue, ApiError> {
    let InnerRequest {
        data,
        shared_state,
        claims,
        ..
    } = request;

    let data = data.unwrap();
    let name = data.name.unwrap();
    let channels = data.channels;
    let tx = shared_state.db_pool.begin().await?;
    let queue = QueuesAM {
        id: Set(Uuid::now_v7()),
        name: Set(name),
        ..Default::default()
    };
    let queue = QueuesE::insert(queue).exec_with_returning(&tx).await?;
    let channels = match channels {
        Some(channels) => channels,
        None => return Ok((queue, vec![]).into()),
    };
    let channels_len = channels.len().clone();
    let db_channels = ChannelsE::find()
        .filter(ChannelsC::Id.is_in(channels.into_iter().map(|channel| channel.id.unwrap())))
        .all(&tx)
        .await?;
    if db_channels.len() != channels_len {
        return Err(ApiError {
            status_code: StatusCode::BAD_REQUEST,
            message: "Some or all channels are not found".to_string(),
        });
    }
    QueuesChannelsAssignmentE::insert_many(db_channels.into_iter().map(|db_channel| {
        QueuesChannelsAssignmentAM {
            queue_id: Set(queue.id),
            channel_id: Set(db_channel.id),
            ..Default::default()
        }
    }))
    .exec(&tx)
    .await?;
    let mut queue_with_assignment = QueuesE::find_by_id(queue.id)
        .find_with_linked(QueuesToChannels)
        .all(&tx)
        .await?;
    tx.commit().await?;
    Ok(queue_with_assignment.remove(0).into())
}

pub async fn replace_queues_channels_assignments(
    request: InnerRequest<Queue>,
) -> Result<Queue, ApiError> {
    let InnerRequest {
        id,
        data,
        shared_state,
        claims,
    } = request;
    let data = data.unwrap();
    let queue_id = data.id.unwrap();
    let channels = data.channels.unwrap();
    let channels_len = channels.len().clone();
    let tx = shared_state.db_pool.begin().await?;
    let queue = QueuesE::find_by_id(queue_id)
        .one(&tx)
        .await?
        .ok_or(ApiError {
            status_code: StatusCode::BAD_REQUEST,
            message: "queue not found".to_string(),
        })?;
    let db_channels = ChannelsE::find()
        .filter(ChannelsC::Id.is_in(channels.into_iter().map(|channel| channel.id.unwrap())))
        .all(&tx)
        .await?;
    if db_channels.len() != channels_len {
        return Err(ApiError {
            status_code: StatusCode::BAD_REQUEST,
            message: "some or all channels are not found".to_string(),
        });
    }
    QueuesChannelsAssignmentE::delete_many()
        .filter(QueuesChannelsAssignmentC::QueueId.eq(queue_id))
        .exec(&tx)
        .await?;
    QueuesChannelsAssignmentE::insert_many(db_channels.into_iter().map(|db_channel| {
        QueuesChannelsAssignmentAM {
            queue_id: Set(queue_id),
            channel_id: Set(db_channel.id),
            ..Default::default()
        }
    }))
    .exec_with_returning(&tx)
    .await?;
    let mut queue_with_assignment = QueuesE::find_by_id(queue.id)
        .find_with_linked(QueuesToChannels)
        .all(&tx)
        .await?;
    tx.commit().await?;
    Ok(queue_with_assignment.remove(0).into())
}

pub async fn enqueue_interaction(
    request: InnerRequest<Interaction>,
) -> Result<Interaction, ApiError> {
    let InnerRequest {
        id,
        data,
        shared_state,
        claims,
    } = request;
    let data = data.unwrap();
    let interaction_id = data.id.unwrap();
    let queue_id = data.queue.unwrap().id.unwrap();
    let priority = data.priority.unwrap_or(DEFAULT_QUEUE_PRIORITY);
    let tx = shared_state.db_pool.begin().await?;
    tx_lock(interaction_id, &tx).await?;
    tx_lock(queue_id, &tx).await?;
    let queue = QueuesE::find_by_id(queue_id)
        .one(&tx)
        .await?
        .ok_or(ApiError {
            status_code: StatusCode::BAD_REQUEST,
            message: "queue not found".to_string(),
        })?;
    let interaction = InteractionsE::find_by_id(interaction_id)
        .find_also_related(ChannelsE)
        .one(&tx)
        .await?
        .ok_or(ApiError {
            status_code: StatusCode::BAD_REQUEST,
            message: "interaction not found".to_string(),
        })?;
    let (interaction, channel) = interaction;
    let channel = channel.ok_or(ApiError {
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        message: "interaction is not mapped to a channel, check logs".to_string(),
    })?;
    validate_interaction_state_change(
        uuid_to_interaction_state(&interaction.state).await?,
        InteractionStates::Enqueued,
    )?;
    let enqueued_interaction = RuntimeInteractionsQueuesAM {
        queue_id: Set(queue.id),
        interaction_id: Set(interaction.id),
        priority: Set(priority),
        ..Default::default()
    };
    RuntimeInteractionsQueuesE::insert(enqueued_interaction)
        .exec_without_returning(&tx)
        .await?;
    let interaction_event = InteractionsEventsAM {
        id: Set(interaction.id),
        old_state: Set(Some(interaction.state)),
        new_state: Set(Some(
            interaction_state_to_uuid(InteractionStates::Enqueued).await?,
        )),
        queue_id: Set(Some(queue.id)),
        ..Default::default()
    };
    InteractionsEventsE::insert(interaction_event)
        .exec_without_returning(&tx)
        .await?;
    let mut interaction = interaction.into_active_model();
    interaction.state = Set(interaction_state_to_uuid(InteractionStates::Enqueued).await?);
    let interaction = InteractionsE::update(interaction).exec(&tx).await?;

    let nats_interaction: Interaction = interaction.clone().into();
    let nats_payload = serde_json::to_string(&ChannelMessage {
        id,
        r#type: ChannelMessageType::Update,
        target: ChannelMessageTarget::Interaction,
        interaction: Some(nats_interaction),
        ..Default::default()
    })
    .unwrap();
    spawn(|| notify_channel_enqueued_interaction(shared_state, channel, nats_payload));

    tx.commit().await?;
    Ok(interaction.into())
}

pub async fn dequeue_interaction(
    request: InnerRequest<Interaction>,
) -> Result<Interaction, ApiError> {
    let InnerRequest {
        id,
        shared_state,
        data,
        claims,
    } = request;
    let data = data.unwrap();
    let interaction_id = data.id.unwrap();

    let tx = shared_state.db_pool.begin().await?;
    tx_lock(interaction_id, &tx).await?;
    let enqueued_interaction = RuntimeInteractionsQueuesE::find()
        .filter(RuntimeInteractionsQueuesC::InteractionId.eq(interaction_id))
        .one(&tx)
        .await?
        .ok_or(ApiError {
            status_code: StatusCode::BAD_REQUEST,
            message: "Interaction is not in a queue".to_string(),
        })?;
    tx_lock(enqueued_interaction.queue_id, &tx).await?;
    let interaction = InteractionsE::find_by_id(interaction_id)
        .one(&tx)
        .await?
        .ok_or(ApiError {
            status_code: StatusCode::BAD_REQUEST,
            message: "interaction not found".to_string(),
        })?;
    validate_interaction_state_change(
        uuid_to_interaction_state(&interaction.state).await?,
        InteractionStates::Dequeued,
    )?;
    RuntimeInteractionsQueuesE::delete_by_id((
        enqueued_interaction.queue_id,
        enqueued_interaction.interaction_id,
    ))
    .exec(&tx)
    .await?;
    let interaction_event = InteractionsEventsAM {
        id: Set(interaction.id),
        old_state: Set(Some(interaction.state)),
        new_state: Set(Some(
            interaction_state_to_uuid(InteractionStates::Dequeued).await?,
        )),
        queue_id: Set(Some(enqueued_interaction.queue_id)),
        ..Default::default()
    };
    InteractionsEventsE::insert(interaction_event)
        .exec_without_returning(&tx)
        .await?;
    let mut interaction = interaction.into_active_model();
    interaction.state = Set(interaction_state_to_uuid(InteractionStates::Dequeued).await?);
    let interaction = InteractionsE::update(interaction).exec(&tx).await?;
    tx.commit().await?;
    Ok(interaction.into())
}
