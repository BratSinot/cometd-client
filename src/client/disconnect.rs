use crate::{
    types::{CometdError, CometdResult, InnerError},
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
    /// # let client = CometdClientBuilder::new("http://[::1]:1025/".parse().unwrap()).build().unwrap();
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
        }])
        .to_string();

        let cookie = self.cookie.swap(None);
        let request_builder =
            self.create_request_builder_with_cookie(&self.disconnect_endpoint, cookie);

        let response = self
            .send_request_response(request_builder, body, CometdError::disconnect_error)
            .await?;

        match response.status() {
            StatusCode::BAD_REQUEST => Ok(()),
            code => Err(CometdError::disconnect_error(InnerError::WrongResponse(
                format!("Unknown status code: {code}").into(),
            ))),
        }
    }
}
