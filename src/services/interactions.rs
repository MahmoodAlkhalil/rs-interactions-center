use crate::db::entities::channels::Entity as ChannelsEntity;
use crate::db::entities::interactions::*;
use crate::db::repo::interactions as InteractionsRepo;
use crate::dtos::interactions::requests::CreateInteraction;
use crate::dtos::interactions::responses::Interaction as InteractionDto;
use crate::dtos::shared::RequestDto;
use crate::shared::errors::{IcError, NoType, WithMetadata};
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
    let interactions = InteractionsRepo::find_all(request.db)
        .await
        .with_metadata(request.id)?;
    let response: Vec<InteractionDto> = interactions.iter().map(|i| i.into()).collect();
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
    Ok(interaction.into())
}
