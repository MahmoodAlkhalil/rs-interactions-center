use std::{collections::HashMap, sync::LazyLock};

use core_engine_const::{interaction_states::InteractionStates, user_states::UserStates};
use tokio::sync::RwLock;
use uuid::Uuid;

pub static INTERACTION_STATE_TO_UUID: LazyLock<RwLock<HashMap<InteractionStates, Uuid>>> =
    LazyLock::new(|| RwLock::new(HashMap::with_capacity(20)));
pub static UUID_TO_INTERACTION_STATE: LazyLock<RwLock<HashMap<Uuid, InteractionStates>>> =
    LazyLock::new(|| RwLock::new(HashMap::with_capacity(20)));

pub static USER_STATE_TO_UUID: LazyLock<RwLock<HashMap<UserStates, Uuid>>> =
    LazyLock::new(|| RwLock::new(HashMap::with_capacity(20)));
pub static UUID_TO_USER_STATE: LazyLock<RwLock<HashMap<Uuid, UserStates>>> =
    LazyLock::new(|| RwLock::new(HashMap::with_capacity(20)));
