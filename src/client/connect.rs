use crate::types::{Advice, Message};
use crate::{
    consts::APPLICATION_JSON, types::InnerError, CometdClient, CometdError, CometdResult,
    ConnectResponse, Data, RequestBuilderExt,
};
use hyper::{header::CONTENT_TYPE, Method, Request};
use serde::de::DeserializeOwned;
use serde_json::json;

impl CometdClient {
    pub async fn connect<Msg>(&self) -> CometdResult<ConnectResponse<Msg>>
    where
        Msg: DeserializeOwned,
    {
        let client_id = self
            .client_id
            .read()
            .await
            .as_ref()
            .cloned()
            .ok_or_else(|| CometdError::connect_error(InnerError::MissingClientId))?;

        let request_builder = Request::builder()
            .uri(&self.connect_endpoint)
            .method(Method::POST)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .set_authentication_header(&*self.access_token.read().await)
            .set_cookie(self.cookie.read().await.clone());

        let id = self.next_id();
        let body = json!([{
          "id": id,
          "channel": "/meta/connect",
          "connectionType": "long-polling",
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
            .map_err(CometdError::connect_error)?;
        let mut messages = serde_json::from_slice::<Vec<Message>>(raw_body.as_ref())
            .map_err(CometdError::connect_error)?;

        if let Some(position) = messages
            .iter()
            .position(|message| message.id.as_ref() == Some(&id))
        {
            let Message {
                successful,
                error,
                advice,
                ..
            } = messages.remove(position);

            if successful == Some(false) {
                Err(CometdError::connect_error(InnerError::WrongResponse(
                    error.unwrap_or_default().into(),
                )))
            } else {
                let reconnect = advice
                    .as_ref()
                    .and_then(Advice::reconnect)
                    .unwrap_or_default();

                let data = messages
                    .into_iter()
                    .map(|message| {
                        let Message { channel, data, .. } = message;
                        let message = data
                            .map(serde_json::from_value::<Msg>)
                            .transpose()
                            .map_err(CometdError::connect_error)?;

                        Ok::<_, CometdError>(Data { channel, message })
                    })
                    .collect::<CometdResult<Vec<_>>>()?;

                Ok(ConnectResponse { reconnect, data })
            }
        } else {
            Err(CometdError::connect_error(InnerError::WrongResponse(
                "The response corresponding request id cannot be found.".into(),
            )))
        }
    }
}
