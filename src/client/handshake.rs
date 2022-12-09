use crate::{
    types::{CometdError, CometdResult, InnerError, Message},
    ArcSwapOptionExt, CometdClient,
};
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
        }])
        .to_string();

        let request_builder = self.create_request_builder(&self.handshake_endpoint);
        let raw_body = self
            .send_request(request_builder, body, CometdError::handshake_error)
            .await?;

        let Message {
            client_id,
            supported_connection_types,
            successful,
            error,
            ..
        } = serde_json::from_slice::<[Message; 1]>(raw_body.as_ref())
            .map(|[message]| message)
            .map_err(CometdError::handshake_error)?;

        if successful == Some(false) {
            Err(CometdError::handshake_error(InnerError::WrongResponse(
                error.unwrap_or_default().into(),
            )))
        } else if !supported_connection_types
            .iter()
            .flatten()
            .any(|connection_type| connection_type == "long-polling")
        {
            let msg = format!(
                "Server doesn't support long-polling mode: `{supported_connection_types:?}`."
            )
            .into();
            Err(CometdError::handshake_error(InnerError::WrongResponse(msg)))
        } else if let Some(client_id) = client_id {
            self.client_id.store_value(client_id);
            Ok(())
        } else {
            Err(CometdError::handshake_error(InnerError::WrongResponse(
                "Missing client_id".into(),
            )))
        }
    }
}
