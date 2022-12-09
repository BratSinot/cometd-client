use crate::{types::InnerError, CometdClient, CometdError, CometdResult};
use hyper::StatusCode;
use serde_json::json;

impl CometdClient {
    /// Send disconnect request.
    ///
    /// # Example
    /// ```rust
    /// # use cometd_client::{CometdClientBuilder, CometdResult};
    /// # let client = CometdClientBuilder::new().endpoint("http://[::1]:1025/").build().unwrap();
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
            .ok_or_else(|| CometdError::connect_error(InnerError::MissingClientId))?;
        let cookie = self.cookie.swap(None);

        let request_builder =
            self.create_request_builder_with_cookie(&self.disconnect_endpoint, cookie);

        let id = self.next_id();
        let body = json!([{
          "id": id,
          "channel": "/meta/disconnect",
          "clientId": client_id
        }])
        .to_string();

        let request = request_builder
            .body(body.into())
            .map_err(CometdError::unexpected_error)?;

        let response = self
            .http_client
            .request(request)
            .await
            .map_err(CometdError::disconnect_error)?;

        match response.status() {
            StatusCode::BAD_REQUEST => Ok(()),
            code => Err(CometdError::disconnect_error(InnerError::WrongResponse(
                format!("Unknown status code: {code}").into(),
            ))),
        }
    }
}
