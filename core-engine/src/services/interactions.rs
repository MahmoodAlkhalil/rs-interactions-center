use std::str::FromStr;

use axum::{Json, http::StatusCode};
use core_engine_const::interaction_states::InteractionStates;
use core_engine_db::entities::channels::Entity as ChannelEntity;
use core_engine_db::entities::interactions::{
    ActiveModel as InteractionsActiveModel, Entity as InteractionsEntity,
};
use core_engine_dto::{Interaction, Request, errors::IcError};
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, EntityTrait, Set, TransactionSession, TransactionTrait,
};
use strum::EnumProperty;
use uuid::Uuid;
pub async fn get_all<B>(request: Request<'_, (), B>) -> Result<Vec<Interaction>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let interactions = InteractionsEntity::find().all(request.db).await?;
    let response: Vec<Interaction> = interactions.into_iter().map(|i| i.into()).collect();
    Ok(response)
}

pub async fn create<B>(request: Request<'_, Interaction, B>) -> Result<Interaction, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let Request { db, data } = request;
    let data = data.unwrap();
    let tx = db.begin().await?;
    ChannelEntity::find_by_id(data.channel.unwrap().id.unwrap())
        .one(&tx)
        .await?
        .ok_or(IcError {
            status_code: StatusCode::BAD_REQUEST,
            message: "channel not found".to_string(),
        })?;
    let interaction = InteractionsActiveModel {
        id: Set(Uuid::now_v7()),
        state: Set(Uuid::from_str(InteractionStates::New.get_str("Id").unwrap()).unwrap()),
        ..Default::default()
    };
    let interaction = InteractionsEntity::insert(interaction)
        .exec_with_returning(&tx)
        .await?;
    tx.commit().await?;
    Ok(interaction.into())
}
