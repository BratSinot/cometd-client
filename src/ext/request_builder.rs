use crate::AccessToken;
use hyper::header::COOKIE;
use hyper::http::request::Builder as RequestBuilder;
use hyper::http::HeaderValue;

pub(crate) trait RequestBuilderExt {
    fn set_authentication_header(self, access_token: &Option<Box<dyn AccessToken>>) -> Self;
    fn set_cookie(self, cookie: Option<HeaderValue>) -> Self;
}

impl RequestBuilderExt for RequestBuilder {
    fn set_authentication_header(mut self, access_token: &Option<Box<dyn AccessToken>>) -> Self {
        for (header, value) in access_token
            .iter()
            .map(Box::as_ref)
            .flat_map(AccessToken::get_authorization_header)
        {
            self = self.header(*header, &**value);
        }

        self
    }

    fn set_cookie(mut self, cookie: Option<HeaderValue>) -> Self {
        if let Some(cookie) = cookie {
            self = self.header(COOKIE, cookie);
        }
        self
    }
}
