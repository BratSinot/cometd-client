use crate::{consts::APPLICATION_JSON, types::AccessToken, CometdClient};
use hyper::{
    header::{CONTENT_TYPE, COOKIE},
    http::request::Builder,
    Method, Request, Uri,
};
use std::sync::Arc;

impl CometdClient {
    #[inline]
    pub(crate) fn create_request_builder(&self, uri: &Uri) -> Builder {
        let mut ret = Request::builder()
            .uri(uri)
            .method(Method::POST)
            .header(CONTENT_TYPE, APPLICATION_JSON);

        // set authorization headers
        #[allow(clippy::pattern_type_mismatch)]
        for (header, value) in self
            .access_token
            .load()
            .iter()
            .map(Arc::as_ref)
            .map(Box::as_ref)
            .flat_map(AccessToken::get_authorization_header)
        {
            ret = ret.header(*header, &**value);
        }

        // set cookies
        if let Some(cookies) = self.cookies_string_cache.load().as_deref() {
            ret = ret.header(COOKIE, cookies.as_ref());
        }

        ret
    }
}
