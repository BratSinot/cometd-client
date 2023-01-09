use crate::{
    types::{Advice, CometdError, CometdResult, Data, InnerError, Message},
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
    /// # let client = CometdClientBuilder::new(&"http://[::1]:1025/".parse().unwrap()).build().unwrap();
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
            .ok_or_else(|| CometdError::connect_error(None, InnerError::MissingClientId))?;
        let id = self.next_id();
        let body = json!([{
          "id": id,
          "channel": "/meta/connect",
          "connectionType": "long-polling",
          "clientId": client_id
        }]);

        let mut messages: Vec<Message> = self
            .send_request(self.connect_endpoint.clone(), &body, |err| {
                CometdError::connect_error(None, err)
            })
            .await?;

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
                Err(CometdError::connect_error(
                    Advice::reconnect(&advice),
                    InnerError::WrongResponse(error.unwrap_or_default().into()),
                ))
            } else {
                let data = messages
                    .into_iter()
                    .map(|message| {
                        let Message {
                            channel,
                            data,
                            advice,
                            ..
                        } = message;
                        let message = data
                            .map(serde_json::from_value::<Msg>)
                            .transpose()
                            .map_err(|err| {
                                CometdError::connect_error(Advice::reconnect(&advice), err)
                            })?;

                        Ok::<_, CometdError>(Data { channel, message })
                    })
                    .collect::<CometdResult<Vec<_>>>()?;

                Ok(data)
            }
        } else {
            Err(CometdError::connect_error(
                None,
                InnerError::WrongResponse(
                    "The response corresponding request id cannot be found.".into(),
                ),
            ))
        }
    }
}
