use crate::{
    types::{Advice, CometdError, CometdResult, InnerError, Message},
    ArcSwapOptionExt, CometdClient,
};
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
    ///     client.handshake().await?;
    /// #   CometdResult::Ok(())
    /// # };
    /// ```
    pub async fn handshake(&self) -> CometdResult<()> {
        let body = json!([{
          "id": self.next_id(),
          "version": "1.0",
          "minimumVersion": "1.0",
          "channel": "/meta/handshake",
          "supportedConnectionTypes": [ "long-polling" ],
          "advice": {
            "timeout": self.timeout_ms,
            "interval": self.interval_ms,
          }
        }]);

        let [Message {
            client_id,
            supported_connection_types,
            successful,
            error,
            advice,
            ..
        }]: [Message; 1] = self
            .send_request(self.handshake_endpoint.clone(), &body, |err| {
                CometdError::handshake_error(None, err)
            })
            .await?;

        if successful == Some(false) {
            Err(CometdError::handshake_error(
                Advice::reconnect(&advice),
                InnerError::WrongResponse(error.unwrap_or_default().into()),
            ))
        } else if !supported_connection_types
            .iter()
            .flatten()
            .any(|connection_type| connection_type == "long-polling")
        {
            let msg = format!(
                "Server doesn't support long-polling mode: `{supported_connection_types:?}`."
            )
            .into();
            Err(CometdError::handshake_error(
                None,
                InnerError::WrongResponse(msg),
            ))
        } else if let Some(client_id) = client_id {
            self.client_id.store_value(client_id);
            Ok(())
        } else {
            Err(CometdError::handshake_error(
                None,
                InnerError::WrongResponse("Missing client_id".into()),
            ))
        }
    }
}
