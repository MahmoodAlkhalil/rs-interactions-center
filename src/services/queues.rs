use crate::db::entities::queues::ActiveModel as QueuesActiveModel;
use crate::db::repo::channels as ChannelsRepo;
use crate::db::repo::interactions as InteractionsRepo;
use crate::db::repo::queues as QueuesRepo;
use crate::dtos::queues::requests::{CreateQueue, EnqueueInteraction, QueueToChannelsMapping};
use crate::dtos::queues::responses::Queue as QueueDto;
use crate::dtos::shared::RequestDto;
use crate::shared::errors::{IcError, NoType, WithMetadata};
use crate::shared::interaction_states::{validate_state_change, InteractionStates};
use sea_orm::prelude::*;
use sea_orm::{ConnectionTrait, Set, TransactionTrait};
pub async fn get_all<B>(request: &RequestDto<'_, NoType, B>) -> Result<Vec<QueueDto>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    Ok(QueuesRepo::find_all(request.db)
        .await
        .with_metadata(request.id)?
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
            name: Set(request.data.as_ref().unwrap().name.clone()),
            ..Default::default()
        },
        request.db,
    )
    .await
    .with_metadata(request.id)?
    .into())
}

pub async fn replace_queues_channels_assignments<B>(
    request: RequestDto<'_, QueueToChannelsMapping, B>,
) -> Result<(), IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let tx = request.db.begin().await.with_metadata(request.id)?;
    let queue_with_channels =
        QueuesRepo::find_queue_with_assigned_channels(request.data.as_ref().unwrap().queue_id, &tx)
            .await
            .with_metadata(request.id)?
            .into_iter()
            .next()
            .ok_or(IcError::from((request.id, "Queue not found")))?;

    let channels =
        ChannelsRepo::find_all_by_ids(request.data.as_ref().unwrap().channels.clone(), &tx)
            .await
            .with_metadata(request.id)?;
    if channels.len() != request.data.as_ref().unwrap().channels.len() {
        return Err(IcError::from((
            request.id,
            "One or more channels not found",
        )));
    }
    QueuesRepo::delete_queue_assigned_channels(
        (
            queue_with_channels.0.id,
            queue_with_channels
                .1
                .iter()
                .map(|item| item.queue_id)
                .collect(),
        ),
        &tx,
    )
    .await
    .with_metadata(request.id)?;
    QueuesRepo::insert_queue_to_channel_assignment(
        queue_with_channels.0.id,
        request.data.as_ref().unwrap().channels.clone(),
        &tx,
    )
    .await
    .with_metadata(request.id)?;
    tx.commit().await.with_metadata(request.id)?;
    Ok(())
}

pub async fn enqueue_interaction<B>(
    request: &RequestDto<'_, EnqueueInteraction, B>,
) -> Result<(), IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let tx = request.db.begin().await.with_metadata(request.id)?;
    let interaction =
        InteractionsRepo::find_by_id(request.data.as_ref().unwrap().interaction_id, request.db)
            .await
            .with_metadata(request.id)?
            .ok_or(IcError::from((request.id, "Interaction not found")))?;
    let queue = QueuesRepo::find_by_id(request.data.as_ref().unwrap().queue_id, request.db)
        .await
        .with_metadata(request.id)?
        .ok_or(IcError::from((request.id, "Queue not found")))?;
    validate_state_change(
        interaction.state.try_into().unwrap(),
        InteractionStates::Enqueued,
    )
    .with_metadata(request.id)?;
    Ok(())
}
