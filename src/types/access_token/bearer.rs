use crate::AccessToken;
use hyper::header::AUTHORIZATION;

#[derive(Debug)]
pub struct Bearer([(&'static str, Box<str>); 1]);

impl Bearer {
    #[inline(always)]
    pub fn new(token: &str) -> Self {
        Self([(
            AUTHORIZATION.as_str(),
            format!("Bearer {token}").into_boxed_str(),
        )])
    }
}

impl AccessToken for Bearer {
    fn get_authorization_header<'a>(&'a self) -> &[(&'static str, Box<str>)] {
        &self.0
    }
}
