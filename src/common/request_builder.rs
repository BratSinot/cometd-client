use crate::{consts::APPLICATION_JSON, types::AccessToken, CometdClientInner};
use hyper::{
    header::{AUTHORIZATION, CONTENT_TYPE, COOKIE},
    http::request::Builder,
    Method, Request, Uri,
};

impl CometdClientInner {
    #[inline]
    pub(crate) fn create_request_builder(&self, uri: &Uri) -> Builder {
        let mut ret = Request::builder()
            .uri(uri)
            .method(Method::POST)
            .header(CONTENT_TYPE, APPLICATION_JSON);

        // set authorization header
        if let Some(token) = self
            .access_token
            .load()
            .as_deref()
            .map(Box::as_ref)
            .map(AccessToken::get_authorization_token)
        {
            ret = ret.header(AUTHORIZATION, token);
        }

        // set cookies
        if let Some(cookies) = self.cookies_string_cache.load().as_deref().map(Box::as_ref) {
            ret = ret.header(COOKIE, cookies);
        }

        ret
    }
}
