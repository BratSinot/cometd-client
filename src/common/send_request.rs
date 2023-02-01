use crate::{
    types::{CometdError, CometdResult, ErrorKind},
    CometdClient,
};
use hyper::{
    body::to_bytes,
    http::{request::Builder, response::Parts},
    Body, HeaderMap, StatusCode,
};
use serde::de::DeserializeOwned;
use std::sync::Arc;

impl CometdClient {
    #[inline]
    pub(crate) async fn send_request_response(
        &self,
        request_builder: Builder,
        body: String,
        kind: ErrorKind,
    ) -> CometdResult<(StatusCode, HeaderMap, Body)> {
        let request = request_builder
            .body(body.into())
            .map_err(CometdError::unexpected)?;

        let (parts, body) = self
            .http_client
            .request(request)
            .await
            .map_err(Arc::new)
            .map_err(|error| CometdError::Request(kind, error))?
            .into_parts();
        let Parts {
            status, headers, ..
        } = parts;

        Ok((status, headers, body))
    }

    #[inline]
    pub(crate) async fn send_request_and_parse_json_body<R: DeserializeOwned>(
        &self,
        request_builder: Builder,
        body: String,
        kind: ErrorKind,
    ) -> CometdResult<R> {
        let (status, headers, body) = self
            .send_request_response(request_builder, body, kind)
            .await?;
        let body = to_bytes(body).await.map(Vec::from);

        self.extract_and_store_cookie(&headers).await;

        if status.is_success() {
            let raw_body = body
                .map_err(Arc::new)
                .map_err(|error| CometdError::FetchBody(kind, error))?;

            serde_json::from_slice::<R>(&raw_body)
                .map_err(Arc::new)
                .map_err(|error| CometdError::ParseBody(kind, error))
        } else {
            Err(CometdError::StatusCode(
                kind,
                status,
                body.unwrap_or_default(),
            ))
        }
    }
}
