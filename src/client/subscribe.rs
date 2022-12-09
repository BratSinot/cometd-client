use crate::types::Message;
use crate::{
    consts::APPLICATION_JSON, types::InnerError, CometdClient, CometdError, CometdResult,
    RequestBuilderExt,
};
use hyper::{header::CONTENT_TYPE, Method, Request};
use serde::Serialize;
use serde_json::json;

impl CometdClient {
    /// Send handshake request.
    ///
    /// # Example
    /// ```rust
    /// # use cometd_client::{CometdClientBuilder, CometdResult};
    /// # let client = CometdClientBuilder::new().endpoint("http://[::1]:1025/").build().unwrap();
    ///
    /// # async {
    ///     client.subscribe(&["/topic0", "/topic1"]).await?;
    /// #   CometdResult::Ok(())
    /// # };
    /// ```
    pub async fn subscribe<Item>(&self, subscriptions: &[Item]) -> CometdResult<()>
    where
        Item: Serialize,
    {
        let client_id = self
            .client_id
            .load_full()
            .ok_or_else(|| CometdError::subscribe_error(InnerError::MissingClientId))?;

        let request_builder = Request::builder()
            .uri(&self.subscribe_endpoint)
            .method(Method::POST)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .set_authentication_header(&self.access_token.load())
            .set_cookie(self.cookie.load_full());

        let body = json!([{
          "id": self.next_id(),
          "channel": "/meta/subscribe",
          "subscription": subscriptions,
          "clientId": client_id
        }])
        .to_string();

        let request = request_builder
            .body(body.into())
            .map_err(CometdError::unexpected_error)?;

        let mut response = self
            .http_client
            .request(request)
            .await
            .map_err(CometdError::subscribe_error)?;
        self.extract_and_store_cookie(&mut response).await;

        let raw_body = hyper::body::to_bytes(response)
            .await
            .map_err(CometdError::subscribe_error)?;
        let Message {
            successful, error, ..
        } = serde_json::from_slice::<[Message; 1]>(raw_body.as_ref())
            .map(|[message]| message)
            .map_err(CometdError::subscribe_error)?;

        if successful == Some(false) {
            Err(CometdError::subscribe_error(InnerError::WrongResponse(
                error.unwrap_or_default().into(),
            )))
        } else {
            Ok(())
        }
    }
}
