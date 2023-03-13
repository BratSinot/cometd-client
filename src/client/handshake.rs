use crate::{
    types::{Advice, CometdError, CometdResult, ErrorKind, Message, Reconnect},
    ArcSwapOptionExt as _, CometdClientInner,
};
use serde_json::json;

impl CometdClientInner {
    pub(crate) async fn handshake(&self) -> CometdResult<()> {
        const KIND: ErrorKind = ErrorKind::Handshake;

        let body = json!([{
          "id": self.next_id(),
          "version": "1.0",
          "minimumVersion": "1.0",
          "channel": "/meta/handshake",
          "supportedConnectionTypes": [ "long-polling" ],
          "advice": {
            "timeout": self.timeout.as_millis(),
            "interval": self.interval.as_millis(),
          }
        }])
        .to_string();

        let request_builder = self.create_request_builder(&self.handshake_endpoint);

        let Message {
            client_id,
            supported_connection_types,
            successful,
            error,
            advice,
            ..
        } = self
            .send_request_and_parse_json_body::<[Message; 1]>(request_builder, body, KIND)
            .await
            .map(|[message]| message)?;

        if successful == Some(false) {
            Err(CometdError::wrong_response(
                KIND,
                Advice::reconnect(advice),
                error.unwrap_or_default(),
            ))
        } else if !supported_connection_types
            .iter()
            .flatten()
            .any(|connection_type| connection_type == "long-polling")
        {
            let msg = format!(
                "Server doesn't support long-polling mode: `{supported_connection_types:?}`."
            );

            Err(CometdError::wrong_response(KIND, Reconnect::None, msg))
        } else if let Some(client_id) = client_id {
            self.client_id.store_value(client_id);

            Ok(())
        } else {
            Err(CometdError::wrong_response(
                KIND,
                Reconnect::None,
                "Missing client_id",
            ))
        }
    }
}
