use crate::types::AccessToken;
use core::fmt::Display;
use hyper::header::AUTHORIZATION;

/// `Bearer` can be used as `AccessToken` for bearer authorization ('authorization: Bearer f0596451-af4d-40f4-a290-b5e8372c110b').
///
/// # Example
/// ```rust
/// # use cometd_client::{types::access_token::Bearer, CometdClientBuilder};
/// # let client = CometdClientBuilder::new(&"http://[::1]:1025/".parse().unwrap()).build().unwrap();
///
///     let access_token = Bearer::new("f0596451-af4d-40f4-a290-b5e8372c110b");
///     client.update_access_token(access_token);
/// ```
#[derive(Debug)]
pub struct Bearer([(&'static str, Box<str>); 1]);

impl Bearer {
    /// Create `Bearer` access token.
    #[inline(always)]
    pub fn new<T: Display>(token: T) -> Self {
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
