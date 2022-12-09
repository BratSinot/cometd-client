use crate::{consts::APPLICATION_JSON, CometdClient, RequestBuilderExt};
use hyper::{header::CONTENT_TYPE, http::request::Builder, Method, Request, Uri};

impl CometdClient {
    #[inline]
    pub(crate) fn create_request_builder(&self, uri: &Uri) -> Builder {
        Request::builder()
            .uri(uri)
            .method(Method::POST)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .set_authentication_header(&self.access_token.load())
            .set_cookie(self.cookie.load_full())
    }
}
