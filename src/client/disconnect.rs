use crate::{
    consts::APPLICATION_JSON, types::InnerError, CometdClient, CometdError, CometdResult,
    RequestBuilderExt,
};
use hyper::{header::CONTENT_TYPE, Method, Request, StatusCode};
use serde_json::json;
use tokio::try_join;

impl CometdClient {
    pub async fn disconnect(&self) -> CometdResult<()> {
        let (client_id, cookie) = try_join!(
            async {
                self.client_id
                    .write()
                    .await
                    .take()
                    .ok_or_else(|| CometdError::connect_error(InnerError::MissingClientId))
            },
            async { Ok(self.cookie.write().await.take()) }
        )?;

        let request_builder = Request::builder()
            .uri(&self.disconnect_endpoint)
            .method(Method::POST)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .set_authentication_header(&*self.access_token.read().await)
            .set_cookie(cookie);

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
