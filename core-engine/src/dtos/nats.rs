use std::vec;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyPair {
    pub seed: String,
    pub pub_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NatsUsersConfig {
    pub authorization: Authorization,
}

impl NatsUsersConfig {
    pub fn new() -> Self {
        NatsUsersConfig {
            authorization: Authorization { users: vec![] },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Authorization {
    pub users: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub nkey: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#pub: Option<Vec<String>>,
}

pub struct CreateNatsUser<'a> {
    pub user_id: &'a str,
    pub pub_key: &'a str,
}
