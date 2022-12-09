use crate::{
    types::{CometdError, CometdResult, Data, InnerError, Message},
    CometdClient,
};
use serde::de::DeserializeOwned;
use serde_json::json;

impl CometdClient {
    /// Send connect request.
    ///
    /// # Example
    /// ```rust
    /// # use cometd_client::{CometdClientBuilder, types::CometdResult};
    /// # let client = CometdClientBuilder::new().endpoint("http://[::1]:1025/").build().unwrap();
    ///
    /// # async {
    ///     let data = client.connect::<serde_json::Value>().await?;
    /// #   CometdResult::Ok(())
    /// # };
    /// ```
    pub async fn connect<Msg>(&self) -> CometdResult<Vec<Data<Msg>>>
    where
        Msg: DeserializeOwned,
    {
        let client_id = self
            .client_id
            .load_full()
            .ok_or_else(|| CometdError::connect_error(InnerError::MissingClientId))?;
        let id = self.next_id();
        let body = json!([{
          "id": id,
          "channel": "/meta/connect",
          "connectionType": "long-polling",
          "clientId": client_id
        }])
        .to_string();

        let request_builder = self.create_request_builder(&self.connect_endpoint);
        let raw_body = self
            .send_request(request_builder, body, CometdError::connect_error)
            .await?;

        let mut messages = serde_json::from_slice::<Vec<Message>>(raw_body.as_ref())
            .map_err(CometdError::connect_error)?;

        if let Some(position) = messages
            .iter()
            .position(|message| message.id.as_ref() == Some(&id))
        {
            let Message {
                successful, error, ..
            } = messages.remove(position);

            if successful == Some(false) {
                Err(CometdError::connect_error(InnerError::WrongResponse(
                    error.unwrap_or_default().into(),
                )))
            } else {
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

                Ok(data)
            }
        } else {
            Err(CometdError::connect_error(InnerError::WrongResponse(
                "The response corresponding request id cannot be found.".into(),
            )))
        }
    }
}
