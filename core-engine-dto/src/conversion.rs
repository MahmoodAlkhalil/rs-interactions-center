use crate::{
    ApiResponse, Channel, Group, Interaction, Queue, Request, Skill, User, UserState,
    errors::IcError,
};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use core_engine_const::interaction_states::InteractionStates;
use core_engine_db::entities::channels::Model as ChannelsM;
use core_engine_db::entities::groups::Model as GroupsM;
use core_engine_db::entities::queues::Model as QueuesM;
use core_engine_db::entities::skills::Model as SkillsM;
use core_engine_db::entities::user_states::Model as UserStatesM;
use core_engine_db::entities::users::Model as UsersM;
use core_engine_db::external_entities::user_states_tree_mv::Model as UserStatesTreeM;
use sea_orm::{ConnectionTrait, TransactionTrait};
use serde::Serialize;

impl From<QueuesM> for Queue {
    fn from(value: QueuesM) -> Self {
        Queue {
            id: Some(value.id),
            name: Some(value.name),
            channels: None,
            groups: None,
            created_at: Some(value.created_at.to_utc()),
            ..Default::default()
        }
    }
}

impl From<(QueuesM, Vec<ChannelsM>)> for Queue {
    fn from(value: (QueuesM, Vec<ChannelsM>)) -> Self {
        Queue {
            id: Some(value.0.id),
            name: Some(value.0.name),
            channels: Some(value.1.into_iter().map(|channel| channel.into()).collect()),
            groups: None,
            created_at: Some(value.0.created_at.to_utc()),
            ..Default::default()
        }
    }
}

impl From<ChannelsM> for Channel {
    fn from(value: ChannelsM) -> Self {
        Channel {
            id: Some(value.id),
            name: Some(value.name),
            created_at: Some(value.created_at.to_utc()),
            ..Default::default()
        }
    }
}

impl From<core_engine_db::entities::groups::Model> for Group {
    fn from(value: core_engine_db::entities::groups::Model) -> Self {
        Group {
            id: Some(value.id),
            name: Some(value.name),
            created_at: Some(value.created_at.to_utc()),
            ..Default::default()
        }
    }
}
impl From<core_engine_db::entities::interactions::Model> for Interaction {
    fn from(value: core_engine_db::entities::interactions::Model) -> Self {
        Interaction {
            id: Some(value.id),
            state: Some(value.state),
            queue: None,
            created_at: Some(value.created_at.to_utc()),
            ..Default::default()
        }
    }
}

impl From<UsersM> for User {
    fn from(value: UsersM) -> Self {
        User {
            id: Some(value.id),
            name: Some(value.name),
            username: Some(value.username),
            groups: None,
            created_at: Some(value.created_at.to_utc()),
            ..Default::default()
        }
    }
}

impl From<SkillsM> for Skill {
    fn from(value: SkillsM) -> Self {
        Skill {
            id: Some(value.id),
            name: Some(value.name),
            parent_id: match value.parent_id {
                Some(data) => Some(data),
                None => None,
            },
            created_at: Some(value.created_at.to_utc()),
            ..Default::default()
        }
    }
}

impl From<UserStatesM> for UserState {
    fn from(value: UserStatesM) -> Self {
        UserState {
            id: Some(value.id),
            name: Some(value.name),
            parent_id: match value.parent_id {
                Some(data) => Some(data),
                None => None,
            },
            created_at: Some(value.created_at.to_utc()),
            ..Default::default()
        }
    }
}

impl From<UserStatesTreeM> for UserState {
    fn from(value: UserStatesTreeM) -> Self {
        UserState {
            id: Some(value.id),
            name: Some(value.name),
            parent_id: match value.parent_id {
                Some(data) => Some(data),
                None => None,
            },
            level: Some(value.level),
            path: Some(value.path),
            full_path: Some(value.full_path),
            created_at: Some(value.created_at.to_utc()),
            ..Default::default()
        }
    }
}

impl<'a, A, B> Request<'a, A, B>
where
    B: ConnectionTrait + TransactionTrait,
{
    pub fn new(data: Option<A>, db: &'a B) -> Self {
        Self { data, db }
    }
}

pub trait IntoApiResponse<T> {
    fn into_api_response(self) -> ApiResponse<T>;
}

impl IntoResponse for IcError {
    fn into_response(self) -> Response {
        let status = self.status_code;
        let body = Json(self);
        (status, body).into_response()
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let status = self.code;
        let body = Json(self.data.unwrap());
        (status, body).into_response()
    }
}

impl<T> IntoApiResponse<T> for Result<T, IcError>
where
    T: Serialize,
{
    fn into_api_response(self) -> ApiResponse<T> {
        match self {
            Ok(data) => ApiResponse {
                message: "SUCCESS".to_string(),
                code: StatusCode::OK,
                data: Some(data),
            },
            Err(error) => ApiResponse {
                message: error.message,
                code: error.status_code,
                data: None,
            },
        }
    }
}
