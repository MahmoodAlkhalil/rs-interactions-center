use crate::db::entities::channels::{Column as ChannelsColumn, Entity as ChannelsEntity};
use crate::db::entities::queues::{ActiveModel as QueuesActiveModel, Entity as QueuesEntity};
use crate::db::entities::queues_channels_assignment::{
    ActiveModel as QueuesChannelsAssignmentActiveModel, Column as QueuesChannelsAssignmentColumn,
    Entity as QueuesChannelsAssignmentEntity,
};
use crate::dtos::queues::requests::{CreateQueue, EnqueueInteraction, QueueToChannelsMapping};
use crate::dtos::queues::responses::Queue as QueueDto;
use crate::dtos::shared::{ApiResponse, RequestDto};
use crate::shared::errors::{IcError, NoType, WithMetadata};
use crate::shared::interaction_states::{validate_state_change, InteractionStates};
use axum::Json;
use sea_orm::prelude::*;
use sea_orm::{ConnectionTrait, QuerySelect, Set, TransactionTrait};

pub async fn get_all<B>(
    request: &RequestDto<'_, NoType, B>,
) -> Result<ApiResponse<Vec<QueueDto>>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let queues = QueuesEntity::find()
        .all(request.db)
        .await
        .with_metadata(request.id)?;
    let response: Vec<QueueDto> = queues.iter().map(|q| q.try_into().unwrap()).collect();
    Ok(ApiResponse {
        id: request.id,
        message: "SUCCESS".to_string(),
        code: 0,
        data: Some(response),
    })
}

pub async fn create<B>(request: RequestDto<'_, CreateQueue, B>) -> Result<Json<QueueDto>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let mut queue = QueuesActiveModel::new();
    queue.id = Set(Uuid::now_v7());
    queue.name = Set(request.data.unwrap().name);
    let queue = queue.insert(request.db).await.with_metadata(request.id)?;
    let response = QueueDto {
        id: queue.id,
        name: queue.name,
        created_at: queue.created_at.to_utc(),
    };
    Ok(Json(response))
}

pub async fn replace_queues_channels_assignments<B>(
    request: RequestDto<'_, QueueToChannelsMapping, B>,
) -> Result<ApiResponse<QueueToChannelsMapping>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let tx = request.db.begin().await.with_metadata(request.id)?;
    let queue = QueuesEntity::find_by_id(request.data.as_ref().unwrap().queue_id)
        .one(&tx)
        .await
        .with_metadata(request.id)?
        .ok_or(IcError {
            id: request.id,
            message: "queue not found".to_string(),
        })?;

    let channels = ChannelsEntity::find()
        .filter(ChannelsColumn::Id.is_in(request.data.as_ref().unwrap().channels.clone()))
        .select_only()
        .column(ChannelsColumn::Id)
        .all(&tx)
        .await
        .with_metadata(request.id)?;
    QueuesChannelsAssignmentEntity::delete_many()
        .filter(QueuesChannelsAssignmentColumn::QueueId.contains(request.data.unwrap().queue_id))
        .exec(&tx)
        .await
        .with_metadata(request.id)?;
    QueuesChannelsAssignmentEntity::insert_many(channels.iter().map(|channel| {
        QueuesChannelsAssignmentActiveModel {
            queue_id: Set(queue.id),
            channel_id: Set(channel.id),
        }
    }))
    .exec(&tx)
    .await
    .with_metadata(request.id)?;
    tx.commit().await.with_metadata(request.id)?;
    Ok(ApiResponse::new_success(request.id, None))
}

pub async fn enqueue_interaction<B>(
    request: &RequestDto<'_, EnqueueInteraction, B>,
) -> Result<(), IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let tx = request.db.begin().await.with_metadata(request.id)?;
    let interaction = crate::db::entities::interactions::Entity::find_by_id(
        request.data.as_ref().unwrap().interaction_id,
    )
    .one(&tx)
    .await
    .with_metadata(request.id)?
    .ok_or(IcError {
        id: request.id,
        message: "interaction not found".to_string(),
    })?;
    let queue = QueuesEntity::find_by_id(request.data.as_ref().unwrap().queue_id)
        .one(&tx)
        .await
        .with_metadata(request.id)?
        .ok_or(IcError {
            id: request.id,
            message: "queue not found".to_string(),
        })?;
    validate_state_change(
        interaction.state.try_into().unwrap(),
        InteractionStates::Enqueued,
    )
    .with_metadata(request.id)?;

    Ok(())
}
