use crate::types::{AccessToken, CometdError, CometdResult};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use std::fmt::Display;

/// `Bearer` can be used as `AccessToken` for bearer authorization ('authorization: Bearer f0596451-af4d-40f4-a290-b5e8372c110b').
///
/// # Example
/// ```rust
/// # use cometd_client::{types::access_token::Bearer, CometdClientBuilder};
/// # let client = CometdClientBuilder::new(&"http://[::1]:1025/".parse().unwrap()).build().unwrap();
///
///     let access_token = Bearer::new("f0596451-af4d-40f4-a290-b5e8372c110b").unwrap();
///     client.update_access_token(access_token);
/// ```
#[derive(Debug)]
pub struct Bearer(HeaderMap);

impl Bearer {
    /// Create `Bearer` access token.
    #[inline(always)]
    pub fn new<T: Display>(token: T) -> CometdResult<Self> {
        let bearer = HeaderValue::try_from(format!("Bearer {token}"))
            .map_err(CometdError::unexpected_error)?;

        Ok(Self(HeaderMap::from_iter([(AUTHORIZATION, bearer)])))
    }
}

impl AccessToken for Bearer {
    fn get_authorization_header(&self) -> HeaderMap {
        self.0.clone()
    }
}
