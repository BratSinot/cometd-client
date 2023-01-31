use crate::{client::Inner, types::*, CometdClient};
use hyper::StatusCode;
use serde_json::json;

impl CometdClient {
    /// Send disconnect request.
    ///
    /// # Example
    /// ```rust
    /// # use cometd_client::{CometdClientBuilder, types::CometdResult};
    /// # use std::error::Error;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// # let client = CometdClientBuilder::new(&"http://[::1]:1025/".parse().unwrap()).build().unwrap();
    /// # async {
    ///     client.disconnect().await?;
    /// #   CometdResult::Ok(())
    /// # };
    /// # }
    /// ```
    pub async fn disconnect(&self) -> CometdResult<()> {
        let client_id = self
            .0
            .client_id
            .swap(None)
            .ok_or_else(|| CometdError::MissingClientId(ErrorKind::Disconnect))?;
        let body = json!([{
          "id": self.0.next_id(),
          "channel": "/meta/disconnect",
          "clientId": client_id
        }])
        .to_string();

        self.0
            .commands_tx
            .send(Command::Disconnect(body))
            .await
            .map_err(CometdError::unexpected)
    }
}

impl Inner {
    pub(crate) async fn _disconnect(&self, body: String) -> CometdResult<()> {
        const KIND: ErrorKind = ErrorKind::Disconnect;

        let request_builder = self.create_request_builder(&self.disconnect_endpoint);

        let status_code = self
            .send_request_response(request_builder, body, KIND)
            .await?
            .0;

        if status_code == StatusCode::BAD_REQUEST {
            Ok(())
        } else {
            Err(CometdError::wrong_response(
                KIND,
                Reconnect::None,
                format!("Unknown status code: {status_code}"),
            ))
        }
    }
}
