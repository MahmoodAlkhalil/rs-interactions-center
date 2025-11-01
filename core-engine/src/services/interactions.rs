use std::str::FromStr;

use axum::{Json, http::StatusCode};
use core_engine_const::interaction_states::InteractionStates;
use core_engine_db::entities::channels::Entity as ChannelEntity;
use core_engine_db::entities::interaction_states::Entity as InteractionStatesE;
use core_engine_db::entities::interactions::{
    ActiveModel as InteractionsActiveModel, Entity as InteractionsEntity,
};
use core_engine_dto::{Interaction, errors::ApiError};
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, DatabaseConnection, EntityTrait, Set, TransactionSession,
    TransactionTrait,
};
use strum::{EnumProperty, VariantArray};
use tokio::time::Instant;
use tower::util::error::optional::None;
use tracing::info;
use uuid::Uuid;

use crate::utils::{
    axum::InnerRequest,
    local_caches::{INTERACTION_STATE_TO_UUID, UUID_TO_INTERACTION_STATE},
};

pub async fn get_all(request: InnerRequest<()>) -> Result<Vec<Interaction>, ApiError> {
    let interactions = InteractionsEntity::find()
        .all(request.shared_state.db_pool.as_ref())
        .await?;
    let response: Vec<Interaction> = interactions.into_iter().map(|i| i.into()).collect();
    Ok(response)
}

pub async fn create(request: InnerRequest<Interaction>) -> Result<Interaction, ApiError> {
    let InnerRequest {
        id,
        shared_state,
        data,
        claims,
        ..
    } = request;
    let data = data.unwrap();
    let tx = shared_state.db_pool.begin().await?;
    ChannelEntity::find_by_id(data.channel.unwrap().id.unwrap())
        .one(&tx)
        .await?
        .ok_or(ApiError {
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

pub async fn init_interaction_states_local_cache(db: &DatabaseConnection) -> Result<(), ApiError> {
    let db_interaction_states = InteractionStatesE::find().all(db).await?;
    let enum_interaction_states = InteractionStates::VARIANTS;

    for db_state in db_interaction_states.into_iter() {
        for enum_state in enum_interaction_states.iter() {
            if db_state.name.as_str() == enum_state.to_string() {
                {
                    INTERACTION_STATE_TO_UUID
                        .write()
                        .await
                        .insert(enum_state.clone(), db_state.id.clone());
                }
                {
                    UUID_TO_INTERACTION_STATE
                        .write()
                        .await
                        .insert(db_state.id.clone(), enum_state.clone());
                }
            }
        }
    }
    info!(
        "prepared interaction states to UUID local hasmap {:?}",
        INTERACTION_STATE_TO_UUID
    );
    info!(
        "prepared UUID to interaction states local hasmap {:?}",
        UUID_TO_INTERACTION_STATE
    );
    Ok(())
}
