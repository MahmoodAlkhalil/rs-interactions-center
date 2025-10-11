use crate::db::entities::interactions::Model;
use core_engine_consts::interaction_states::InteractionStates;
use serde::Serialize;

#[derive(Serialize)]
pub struct Interaction {
    pub id: uuid::Uuid,
    pub state: String,
    pub state_id: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<Model> for Interaction {
    fn from(value: Model) -> Self {
        Interaction {
            id: value.id,
            state: InteractionStates::try_from(value.state)
                .unwrap()
                .to_string(),
            state_id: value.state,
            created_at: value.created_at.to_utc(),
        }
    }
}

impl From<&Model> for Interaction {
    fn from(value: &Model) -> Self {
        Interaction {
            id: value.id,
            state: InteractionStates::try_from(value.state)
                .unwrap()
                .to_string(),
            state_id: value.state,
            created_at: value.created_at.to_utc(),
        }
    }
}
