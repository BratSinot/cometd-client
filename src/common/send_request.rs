use crate::{
    types::{CometdError, CometdResult},
    CometdClient,
};
use hyper::{body::Bytes, http::request::Builder, Body, Error as HyperError, Response};

impl CometdClient {
    #[inline]
    pub(crate) async fn send_request_response(
        &self,
        request_builder: Builder,
        body: String,
        map_err: impl Fn(HyperError) -> CometdError + Send,
    ) -> CometdResult<Response<Body>> {
        let request = request_builder
            .body(body.into())
            .map_err(CometdError::unexpected_error)?;

        self.http_client.request(request).await.map_err(map_err)
    }

    #[inline]
    pub(crate) async fn send_request(
        &self,
        request_builder: Builder,
        body: String,
        map_err: impl Fn(HyperError) -> CometdError + Copy + Send,
    ) -> CometdResult<Bytes> {
        let response = self
            .send_request_response(request_builder, body, map_err)
            .await?;
        self.extract_and_store_cookie(&response).await;

        hyper::body::to_bytes(response).await.map_err(map_err)
    }
}
