use crate::{
    types::{CometdError, CometdResult},
    CometdClient,
};
use reqwest::{Error as ReqwestError, Response};
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

impl CometdClient {
    #[inline]
    pub(crate) async fn send_request_response(
        &self,
        endpoint: Url,
        body: &impl Serialize,
        map_err: impl Fn(ReqwestError) -> CometdError,
    ) -> CometdResult<Response> {
        let headers = self
            .access_token
            .load()
            .as_deref()
            .map(|token| token.get_authorization_header())
            .unwrap_or_default();

        self.http_client
            .post(endpoint)
            .headers(headers)
            .json(body)
            .send()
            .await
            .map_err(map_err)
    }

    #[inline]
    pub(crate) async fn send_request<T: DeserializeOwned>(
        &self,
        endpoint: Url,
        body: &impl Serialize,
        map_err: impl Fn(ReqwestError) -> CometdError + Copy,
    ) -> CometdResult<T> {
        self.send_request_response(endpoint, body, map_err)
            .await?
            .json()
            .await
            .map_err(map_err)
    }
}
