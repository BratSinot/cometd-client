use crate::{
    types::{Advice, CometdError, CometdResult, ErrorKind, Message},
    CometdClientInner,
};
use serde_json::{json, Value as JsonValue};

impl CometdClientInner {
    pub(crate) async fn subscribe(&self, subscriptions: &JsonValue) -> CometdResult<()> {
        const KIND: ErrorKind = ErrorKind::Subscribe;

        let client_id = self
            .client_id
            .load_full()
            .ok_or_else(|| CometdError::MissingClientId(KIND))?;
        let body = json!([{
          "id": self.next_id(),
          "channel": "/meta/subscribe",
          "subscription": subscriptions,
          "clientId": *client_id
        }])
        .to_string();

        let request_builder = self.create_request_builder(&self.subscribe_endpoint);
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
