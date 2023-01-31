use crate::{client::Inner, types::*, ArcSwapOptionExt, CometdClient};
use serde_json::json;

impl CometdClient {
    /// Send handshake request.
    ///
    /// # Example
    /// ```rust
    /// # use cometd_client::{CometdClientBuilder, types::CometdResult};
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> CometdResult<()> {
    /// # let client = CometdClientBuilder::new(&"http://[::1]:1025/".parse().unwrap()).build().unwrap();
    ///     client.handshake().await?;
    /// #   Ok(())
    /// # }
    /// ```
    pub async fn handshake(&self) -> CometdResult<()> {
        self.0
            .commands_tx
            .send(Command::Handshake)
            .await
            .map_err(CometdError::unexpected)
    }
}

impl Inner {
    pub(crate) async fn _handshake(&self) -> CometdResult<()> {
        const KIND: ErrorKind = ErrorKind::Handshake;

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
