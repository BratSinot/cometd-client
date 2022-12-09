use crate::AccessToken;
use hyper::{
    header::COOKIE,
    http::{request::Builder as RequestBuilder, HeaderValue},
};
use std::sync::Arc;

pub(crate) trait RequestBuilderExt {
    fn set_authentication_header(self, access_token: &Option<Arc<Box<dyn AccessToken>>>) -> Self;
    fn set_cookie(self, cookie: Option<Arc<HeaderValue>>) -> Self;
}

impl RequestBuilderExt for RequestBuilder {
    fn set_authentication_header(
        mut self,
        access_token: &Option<Arc<Box<dyn AccessToken>>>,
    ) -> Self {
        for (header, value) in access_token
            .iter()
            .map(Arc::as_ref)
            .map(Box::as_ref)
            .flat_map(AccessToken::get_authorization_header)
        {
            self = self.header(*header, &**value);
        }

        self
    }

    fn set_cookie(mut self, cookie: Option<Arc<HeaderValue>>) -> Self {
        if let Some(cookie) = cookie
            .map(Arc::try_unwrap)
            .map(|cookie| cookie.unwrap_or_else(|err| err.as_ref().clone()))
        {
            self = self.header(COOKIE, cookie);
        }
        self
    }
}
