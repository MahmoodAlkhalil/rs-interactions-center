use crate::db::entities::channels::Entity as ChannelEntity;
use crate::db::entities::interactions::{
    ActiveModel as InteractionsActiveModel, Entity as InteractionsEntity,
};
use crate::dtos::interactions::requests::CreateInteraction;
use crate::dtos::interactions::responses::Interaction as InteractionDto;
use crate::dtos::shared::RequestDto;
use crate::utils::errors::{IcError, NoType};
use core_engine_consts::interaction_states::InteractionStates;
use axum::http::StatusCode;
use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, Set, TransactionSession, TransactionTrait};
use uuid::Uuid;

pub async fn get_all<B>(request: &RequestDto<'_, NoType, B>) -> Result<Vec<InteractionDto>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let interactions = InteractionsEntity::find().all(request.db).await?;
    let response: Vec<InteractionDto> = interactions.iter().map(|i| i.into()).collect();
    Ok(response)
}

pub async fn create<B>(
    request: &RequestDto<'_, CreateInteraction, B>,
) -> Result<InteractionDto, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let tx = request.db.begin().await?;
    ChannelEntity::find_by_id(request.data.as_ref().unwrap().channel_id)
        .one(request.db)
        .await?
        .ok_or(IcError {
            status_code: StatusCode::BAD_REQUEST,
            message: "channel not found".to_string(),
        })?;
    let interaction = InteractionsActiveModel {
        id: Set(Uuid::now_v7()),
        state: Set(InteractionStates::New.into()),
        ..Default::default()
    };
    let interaction = InteractionsEntity::insert(interaction)
        .exec_with_returning(request.db)
        .await?;
    tx.commit().await?;
    Ok(interaction.into())
}
