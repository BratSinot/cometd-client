use crate::{consts::APPLICATION_JSON, AccessToken, CometdClient};
use hyper::{
    header::{CONTENT_TYPE, COOKIE},
    http::{request::Builder, HeaderValue},
    Method, Request, Uri,
};
use std::sync::Arc;

impl CometdClient {
    #[inline(always)]
    pub(crate) fn create_request_builder(&self, uri: &Uri) -> Builder {
        self.create_request_builder_with_cookie(uri, self.cookie.load_full())
    }

    #[inline]
    pub(crate) fn create_request_builder_with_cookie(
        &self,
        uri: &Uri,
        cookie: Option<Arc<HeaderValue>>,
    ) -> Builder {
        let mut ret = Request::builder()
            .uri(uri)
            .method(Method::POST)
            .header(CONTENT_TYPE, APPLICATION_JSON);

        // set authorization headers
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
        if let Some(cookie) = cookie
            .map(Arc::try_unwrap)
            .map(|cookie| cookie.unwrap_or_else(|err| err.as_ref().clone()))
        {
            ret = ret.header(COOKIE, cookie);
        }

        ret
    }
}
