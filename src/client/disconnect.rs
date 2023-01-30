use crate::types::Reconnect;
use crate::{
    types::{CometdError, CometdResult, ErrorKind},
    CometdClient,
};
use hyper::StatusCode;
use serde_json::json;

impl CometdClient {
    /// Send disconnect request.
    ///
    /// # Example
    /// ```rust
    /// # use cometd_client::{CometdClientBuilder, types::CometdResult};
    /// # let client = CometdClientBuilder::new(&"http://[::1]:1025/".parse().unwrap()).build().unwrap();
    ///
    /// # async {
    ///     client.disconnect().await?;
    /// #   CometdResult::Ok(())
    /// # };
    /// ```
    pub async fn disconnect(&self) -> CometdResult<()> {
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
