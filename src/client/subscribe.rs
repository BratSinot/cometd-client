use crate::{
    types::{Advice, CometdError, CometdResult, ErrorKind, Message},
    CometdClient,
};
use serde::Serialize;
use serde_json::json;

impl CometdClient {
    /// Send handshake request.
    ///
    /// # Example
    /// ```rust
    /// # use cometd_client::{CometdClientBuilder, types::CometdResult};
    /// # let client = CometdClientBuilder::new(&"http://[::1]:1025/".parse().unwrap()).build().unwrap();
    ///
    /// # async {
    ///     client.subscribe(&["/topic0", "/topic1"]).await?;
    /// #   CometdResult::Ok(())
    /// # };
    /// ```
    pub async fn subscribe(
        &self,
        subscriptions: &[impl Serialize + Send + Sync],
    ) -> CometdResult<()> {
        const KIND: ErrorKind = ErrorKind::Subscribe;

        let client_id = self
            .client_id
            .load_full()
            .ok_or_else(|| CometdError::MissingClientId(KIND))?;
        let body = json!([{
          "id": self.next_id(),
          "channel": "/meta/subscribe",
          "subscription": subscriptions,
          "clientId": client_id
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
