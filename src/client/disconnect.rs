use crate::{
    types::{CometdError, CometdResult, InnerError},
    CometdClient,
};
use reqwest::StatusCode;
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
        let client_id = self
            .client_id
            .swap(None)
            .ok_or_else(|| CometdError::connect_error(None, InnerError::MissingClientId))?;
        let body = json!([{
          "id": self.next_id(),
          "channel": "/meta/disconnect",
          "clientId": client_id
        }]);

        let response = self
            .send_request_response(
                self.disconnect_endpoint.clone(),
                &body,
                CometdError::disconnect_error,
            )
            .await?;

        match response.status() {
            StatusCode::BAD_REQUEST => Ok(()),
            code => Err(CometdError::disconnect_error(InnerError::WrongResponse(
                format!("Unknown status code: {code}").into(),
            ))),
        }
    }
}
