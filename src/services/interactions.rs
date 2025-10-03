use crate::db::entities::channels::Entity as ChannelsEntity;
use crate::db::entities::interactions::Entity as InteractionsEntity;
use crate::db::entities::interactions::*;
use crate::db::entities::queues::Entity as QueuesEntity;
use crate::db::repo::interactions as InteractionsRepo;
use crate::dtos::interactions::requests::CreateInteraction;
use crate::dtos::interactions::responses::Interaction as InteractionDto;
use crate::dtos::queues::requests::EnqueueInteraction as EnqueueInteractionDto;
use crate::dtos::shared::RequestDto;
use crate::shared::errors::{IcError, NoType, WithMetadata};
use crate::shared::interaction_states::{validate_state_change, InteractionStates};
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ConnectionTrait, EntityTrait, Set, TransactionTrait,
};
use uuid::Uuid;

pub async fn template<B>(request: &RequestDto<'_, CreateInteraction, B>) -> Result<(), IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    Ok(())
}

pub async fn get_all<B>(request: &RequestDto<'_, NoType, B>) -> Result<Vec<InteractionDto>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let interactions = InteractionsRepo::find_all(request).await?;
    let response: Vec<InteractionDto> =
        interactions.iter().map(|i| i.try_into().unwrap()).collect();
    Ok(response)
}

pub async fn create<B>(
    request: &RequestDto<'_, CreateInteraction, B>,
) -> Result<InteractionDto, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let tx = request.db.begin().await.with_metadata(request.id)?;
    ChannelsEntity::find_by_id(request.data.as_ref().unwrap().channel_id)
        .one(&tx)
        .await
        .with_metadata(request.id)?
        .ok_or(IcError {
            id: request.id,
            message: "channel not found".to_string(),
        })?;
    let mut interaction = ActiveModel::new();
    interaction.id = Set(Uuid::now_v7());
    interaction.channel_id = Set(request.data.as_ref().unwrap().channel_id);
    let interaction = interaction.insert(&tx).await.with_metadata(request.id)?;
    tx.commit().await.with_metadata(request.id)?;
    let response = InteractionDto {
        id: interaction.id,
        created_at: interaction.created_at.to_utc(),
    };
    Ok(response)
}

pub async fn enqueue<B>(request: &RequestDto<'_, EnqueueInteractionDto, B>) -> Result<(), IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let tx = request.db.begin().await.with_metadata(request.id)?;
    let interaction = InteractionsEntity::find_by_id(request.data.as_ref().unwrap().interaction_id)
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
