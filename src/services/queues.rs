use crate::db::entities::queues::ActiveModel as QueuesActiveModel;
use crate::db::repo::channels as ChannelsRepo;
use crate::db::repo::interactions as InteractionsRepo;
use crate::db::repo::queues as QueuesRepo;
use crate::dtos::queues::requests::{CreateQueue, EnqueueInteraction, QueueToChannelsMapping};
use crate::dtos::queues::responses::Queue as QueueDto;
use crate::dtos::shared::RequestDto;
use crate::utils::errors::{IcError, NoType};
use crate::utils::interaction_states::{validate_state_change, InteractionStates};
use axum::http::StatusCode;
use sea_orm::prelude::*;
use sea_orm::{ConnectionTrait, Set, TransactionTrait};
pub async fn get_all<B>(request: &RequestDto<'_, NoType, B>) -> Result<Vec<QueueDto>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    Ok(QueuesRepo::find_all(request.db)
        .await?
        .iter()
        .map(|q| q.try_into().unwrap())
        .collect())
}

pub async fn create<B>(request: &RequestDto<'_, CreateQueue, B>) -> Result<QueueDto, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    Ok(QueuesRepo::insert(
        QueuesActiveModel {
            id: Set(Uuid::now_v7()),
            name: Set(request.request.as_ref().unwrap().name.clone()),
            ..Default::default()
        },
        request.db,
    )
    .await?
    .into())
}

pub async fn replace_queues_channels_assignments<B>(
    request: RequestDto<'_, QueueToChannelsMapping, B>,
) -> Result<(), IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let tx = request.db.begin().await?;
    let queue_with_channels = QueuesRepo::find_queue_with_assigned_channels(
        &request.request.as_ref().unwrap().queue_id,
        &tx,
    )
    .await
    .into_iter()
    .next()
    .ok_or(IcError {
        status_code: StatusCode::BAD_REQUEST,
        message: "Queue not found".to_string(),
    })?
    .into_iter()
    .next()
    .ok_or(IcError {
        status_code: StatusCode::BAD_REQUEST,
        message: "Queue not found".to_string(),
    })?;
    let channels =
        ChannelsRepo::find_all_by_ids(request.request.as_ref().unwrap().channels.clone(), &tx)
            .await?;
    if channels.len() != request.request.as_ref().unwrap().channels.len() {
        return Err(IcError {
            status_code: StatusCode::BAD_REQUEST,
            message: "One or more channels not found".to_string(),
        });
    }
    QueuesRepo::delete_queue_to_channels_assignment(&queue_with_channels, &tx).await?;
    QueuesRepo::insert_queue_to_channel_assignment(
        &queue_with_channels.0.id,
        &request.request.as_ref().unwrap().channels,
        &tx,
    )
    .await?;
    tx.commit().await?;
    Ok(())
}

pub async fn enqueue_interaction<B>(
    request: &RequestDto<'_, EnqueueInteraction, B>,
) -> Result<(), IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let tx = request.db.begin().await?;
    let interaction =
        InteractionsRepo::find_by_id(request.request.as_ref().unwrap().interaction_id, request.db)
            .await?
            .ok_or(IcError {
                status_code: StatusCode::BAD_REQUEST,
                message: "Interaction not found".to_string(),
            })?;
    let queue = QueuesRepo::find_by_id(&request.request.as_ref().unwrap().queue_id, request.db)
        .await?
        .ok_or(IcError {
            status_code: StatusCode::BAD_REQUEST,
            message: "Queue not found".to_string(),
        })?;
    validate_state_change(
        interaction.state.try_into().unwrap(),
        InteractionStates::Enqueued,
    )?;
    Ok(())
}
