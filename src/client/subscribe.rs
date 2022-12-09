use crate::{
    types::{CometdError, CometdResult, InnerError, Message},
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
    /// # let client = CometdClientBuilder::new().endpoint("http://[::1]:1025/").build().unwrap();
    ///
    /// # async {
    ///     client.subscribe(&["/topic0", "/topic1"]).await?;
    /// #   CometdResult::Ok(())
    /// # };
    /// ```
    pub async fn subscribe<Item>(&self, subscriptions: &[Item]) -> CometdResult<()>
    where
        Item: Serialize,
    {
        let client_id = self
            .client_id
            .load_full()
            .ok_or_else(|| CometdError::subscribe_error(InnerError::MissingClientId))?;
        let body = json!([{
          "id": self.next_id(),
          "channel": "/meta/subscribe",
          "subscription": subscriptions,
          "clientId": client_id
        }])
        .to_string();

        let request_builder = self.create_request_builder(&self.subscribe_endpoint);
        let raw_body = self
            .send_request(request_builder, body, CometdError::subscribe_error)
            .await?;

        let Message {
            successful, error, ..
        } = serde_json::from_slice::<[Message; 1]>(raw_body.as_ref())
            .map(|[message]| message)
            .map_err(CometdError::subscribe_error)?;

        if successful == Some(false) {
            Err(CometdError::subscribe_error(InnerError::WrongResponse(
                error.unwrap_or_default().into(),
            )))
        } else {
            Ok(())
        }
    }
}
