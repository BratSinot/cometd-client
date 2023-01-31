use crate::{client::Inner, types::*};
use serde_json::json;

impl Inner {
    pub(crate) async fn _connect(&self) -> CometdResult<Vec<Data>> {
        const KIND: ErrorKind = ErrorKind::Connect;

        let client_id = self
            .client_id
            .load_full()
            .ok_or(CometdError::MissingClientId(KIND))?;
        let id = self.next_id();
        let body = json!([{
          "id": id,
          "channel": "/meta/connect",
          "connectionType": "long-polling",
          "clientId": client_id
        }])
        .to_string();

        let request_builder = self.create_request_builder(&self.connect_endpoint);

        let mut messages = self
            .send_request_and_parse_json_body::<Vec<Message>>(request_builder, body, KIND)
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
                Err(CometdError::wrong_response(
                    KIND,
                    Advice::reconnect(advice),
                    error.unwrap_or_default(),
                ))
            } else {
                let data = messages
                    .into_iter()
                    .map(|message| {
                        let Message { channel, data, .. } = message;
                        let message = data;

                        Ok::<_, CometdError>(Data { channel, message })
                    })
                    .collect::<CometdResult<Vec<_>>>()?;

                Ok(data)
            }
        } else {
            Err(CometdError::wrong_response(
                KIND,
                Reconnect::None,
                "The response corresponding request id cannot be found.",
            ))
        }
    }
}
