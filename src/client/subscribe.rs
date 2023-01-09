use crate::{
    types::{Advice, CometdError, CometdResult, InnerError, Message},
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
    pub async fn subscribe(&self, subscriptions: &[impl Serialize]) -> CometdResult<()> {
        let client_id = self
            .client_id
            .load_full()
            .ok_or_else(|| CometdError::subscribe_error(None, InnerError::MissingClientId))?;
        let body = json!([{
          "id": self.next_id(),
          "channel": "/meta/subscribe",
          "subscription": subscriptions,
          "clientId": client_id
        }]);

        let [Message {
            successful,
            error,
            advice,
            ..
        }]: [Message; 1] = self
            .send_request(self.subscribe_endpoint.clone(), &body, |err| {
                CometdError::subscribe_error(None, err)
            })
            .await?;

        if successful == Some(false) {
            Err(CometdError::subscribe_error(
                Advice::reconnect(&advice),
                InnerError::WrongResponse(error.unwrap_or_default().into()),
            ))
        } else {
            Ok(())
        }
    }
}
