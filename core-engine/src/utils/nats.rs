use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NatsConfig {
    pub listen: Option<String>,
    pub debug: Option<bool>,
    pub trace: Option<bool>,
    pub logtime: Option<bool>,
    pub websocket: Option<WebSocketConfig>,
    pub max_connections: Option<u32>,
    pub ping_interval: Option<String>,
    pub ping_max: Option<u32>,
    pub operator: Option<String>,
    pub system_account: Option<String>,
    pub resolver: Option<String>,
    pub resolver_preload: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WebSocketConfig {
    pub listen: Option<String>,
    pub no_tls: Option<bool>,
    pub same_origin: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ResolverConfig {
    #[serde(rename = "type")]
    pub resolver_type: Option<String>,
    pub dir: Option<String>,
    pub allow_delete: Option<bool>,
    pub interval: Option<String>,
    pub timeout: Option<String>,
}

impl NatsConfig {
    pub fn from_hcl(input: &str) -> Result<Self, hcl::Error> {
        hcl::from_str(input)
    }

    pub fn to_hcl(&self) -> Result<String, hcl::Error> {
        hcl::to_string(self)
    }
}
