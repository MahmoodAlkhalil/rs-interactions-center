use crate::db::entities::interactions::{
    Column as InteractionsColumns, Entity as InteractionsEntity,
};
use crate::db::entities::queues::{Column as QueuesColumns, Entity as QueuesEntity};
use crate::db::entities::runtime_interactions_queues::ActiveModel as RuntimeInteractionsQueuesActiveModel;
use crate::dtos::queues::requests::EnqueueInteraction;
use crate::dtos::shared::{ApiResponse, RequestDto};
use crate::shared::errors::{IcError, NoType, WithMetadata};
use axum::Json;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ConnectionTrait, EntityTrait, QuerySelect, Set,
    TransactionTrait,
};

pub async fn enqueue<B>(
    request: RequestDto<'_, EnqueueInteraction, B>,
) -> Result<Json<ApiResponse<NoType>>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let tx = request.db.begin().await.with_metadata(request.id)?;
    let interaction = InteractionsEntity::find_by_id(request.data.as_ref().unwrap().interaction_id)
        .select_only()
        .column(InteractionsColumns::Id)
        .one(&tx)
        .await
        .with_metadata(request.id)?
        .ok_or(IcError {
            id: request.id,
            message: "interaction not found".to_string(),
        })?;
    QueuesEntity::find_by_id(request.data.as_ref().unwrap().queue_id)
        .select_only()
        .column(QueuesColumns::Id)
        .one(&tx)
        .await
        .with_metadata(request.id)?
        .ok_or(IcError {
            id: request.id,
            message: "queue not found".to_string(),
        })?;
    let mut enqueued_interaction = RuntimeInteractionsQueuesActiveModel::new();
    enqueued_interaction.interaction_id = Set(request.data.as_ref().unwrap().interaction_id);
    enqueued_interaction.queue_id = Set(request.data.as_ref().unwrap().queue_id);
    enqueued_interaction.priority = Set(request.data.as_ref().unwrap().priority.unwrap_or(50));
    enqueued_interaction
        .insert(&tx)
        .await
        .with_metadata(request.id)?;
    Ok(Json(ApiResponse {
        id: request.id,
        message: "SUCCESS".to_string(),
        code: 0,
        data: None,
    }))
}
