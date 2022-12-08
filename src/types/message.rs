use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_with::skip_serializing_none;
use std::sync::Arc;

#[skip_serializing_none]
#[derive(Debug, Default, Deserialize, Serialize)]
pub(crate) struct Message {
    pub(crate) id: Option<String>,
    pub(crate) version: Option<String>,
    #[serde(rename = "minimumVersion")]
    pub(crate) minimum_version: Option<String>,
    pub(crate) channel: Option<String>,
    #[serde(rename = "clientId")]
    pub(crate) client_id: Option<Arc<str>>,
    #[serde(rename = "supportedConnectionTypes")]
    pub(crate) supported_connection_types: Option<Vec<String>>,
    pub(crate) data: Option<JsonValue>,
    pub(crate) successful: Option<bool>,
    pub(crate) error: Option<String>,
    pub(crate) advice: Option<Advice>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct Advice {
    interval: Option<u64>,
    reconnect: Option<Reconnect>,
    timeout: Option<u64>,
}

impl Advice {
    #[inline(always)]
    pub(crate) fn reconnect(&self) -> Option<Reconnect> {
        self.reconnect
    }
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Reconnect {
    Retry,
    Handshake,
    #[default]
    None,
}
