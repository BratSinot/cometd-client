use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use serde_with::skip_serializing_none;

/// Contains channel name of message received from cometd server.
#[derive(Debug)]
pub struct Data<Msg> {
    /// Channel name from which was received message.
    pub channel: Option<String>,
    /// Received message.
    pub message: Option<Msg>,
}

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
    pub(crate) advice: Option<Advice>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct Advice {
    reconnect: Option<Reconnect>,
    //interval: Option<u64>,
    //timeout: Option<u64>,
}

impl Advice {
    #[inline(always)]
    pub(crate) fn reconnect(this: &Option<Self>) -> Option<Reconnect> {
        this.as_ref().and_then(|advice| advice.reconnect)
    }
}

/// Advice what to do on error.
#[allow(missing_docs)]
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Reconnect {
    #[default]
    None,
    Handshake,
    Retry,
}
