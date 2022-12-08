use crate::{
    consts::APPLICATION_JSON,
    types::{InnerError, Message},
    CometdClient, CometdError, CometdResult, RequestBuilderExt,
};
use hyper::{header::CONTENT_TYPE, Method, Request};
use serde_json::json;

impl CometdClient {
    pub async fn handshake(&self) -> CometdResult<()> {
        let request_builder = Request::builder()
            .uri(&self.handshake_endpoint)
            .method(Method::POST)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .set_authentication_header(&*self.access_token.read().await);

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

        let request = request_builder
            .body(body.into())
            .map_err(CometdError::unexpected_error)?;

        let mut response = self
            .http_client
            .request(request)
            .await
            .map_err(CometdError::handshake_error)?;
        self.extract_and_store_cookie(&mut response).await;

        let raw_body = hyper::body::to_bytes(response)
            .await
            .map_err(CometdError::handshake_error)?;

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
            *self.client_id.write().await = Some(client_id);
            Ok(())
        } else {
            Err(CometdError::handshake_error(InnerError::WrongResponse(
                "Missing client_id".into(),
            )))
        }
    }
}
