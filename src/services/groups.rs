use crate::db::entities::groups::ActiveModel as GroupsActiveModel;
use crate::dtos::groups::requests::CreateGroup;
use crate::dtos::groups::responses::Group as GroupDto;
use crate::dtos::shared::{ApiResponse, RequestDto};
use crate::shared::{IcError, WithMetadata};
use axum::Json;
use sea_orm::ActiveModelTrait;
use sea_orm::{ActiveModelBehavior, ConnectionTrait, TransactionTrait};
use uuid::Uuid;

pub async fn create<B>(
    request: RequestDto<'_, CreateGroup, B>,
) -> Result<Json<ApiResponse<GroupDto>>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let mut group = GroupsActiveModel::new();
    group.id = sea_orm::Set(Uuid::now_v7());
    let group = group.insert(request.db).await.with_metadata(request.id)?;
    let response = GroupDto {
        id: group.id,
        created_at: group.created_at.to_utc(),
    };
    Ok(Json(ApiResponse {
        id: request.id,
        message: "SUCCESS".to_string(),
        code: 0,
        data: Some(response),
    }))
}
