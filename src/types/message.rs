use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Default, Deserialize, Serialize)]
pub(crate) struct Message {
    pub(crate) id: Option<String>,
    pub(crate) version: Option<String>,
    #[serde(rename = "minimumVersion")]
    pub(crate) minimum_version: Option<String>,
    pub(crate) channel: Option<String>,
    #[serde(rename = "clientId")]
    pub(crate) client_id: Option<Box<str>>,
    #[serde(rename = "supportedConnectionTypes")]
    pub(crate) supported_connection_types: Option<Vec<String>>,
    pub(crate) data: Option<JsonValue>,
    pub(crate) successful: Option<bool>,
    pub(crate) error: Option<String>,
    // TODO: try to parse on errors
    //pub(crate) advice: Option<Advice>,
}

/*#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct Advice {
    interval: Option<u64>,
    reconnect: Option<Reconnect>,
    timeout: Option<u64>,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Reconnect {
    Retry,
    Handshake,
    #[default]
    None,
}*/
