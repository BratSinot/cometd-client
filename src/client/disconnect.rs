use crate::{
    types::{Advice, CometdError, CometdResult, ErrorKind, Message, Reconnect},
    CometdClientInner,
};
use serde_json::json;

impl CometdClientInner {
    pub(crate) async fn disconnect(&self) -> CometdResult<()> {
        const KIND: ErrorKind = ErrorKind::Disconnect;

        let client_id = self
            .client_id
            .swap(None)
            .ok_or_else(|| CometdError::MissingClientId(KIND))?;
        let body = json!([{
          "id": self.next_id(),
          "channel": "/meta/disconnect",
          "clientId": client_id
        }])
        .to_string();

        let request_builder = self.create_request_builder(&self.disconnect_endpoint);

        let Message {
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
        } else {
            Ok(())
        }
    }
}
