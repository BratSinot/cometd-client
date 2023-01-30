use crate::types::{AccessToken, CometdError, CometdResult};
use base64::{
    encoded_len, engine::general_purpose::STANDARD, write::EncoderWriter as Base64Writer,
};
use hyper::header::AUTHORIZATION;
use std::io::Write;

const BASIC: &[u8] = b"Basic ";

/// `Basic` can be used as `AccessToken` for basic authorization ('authorization: Basic VmFzeWE6UGV0eWE=').
///
/// # Example
/// ```rust
/// # use cometd_client::{types::access_token::Basic, CometdClientBuilder};
/// # let client = CometdClientBuilder::new(&"http://[::1]:1025/".parse().unwrap()).build().unwrap();
///
///     let access_token = Basic::create("username", Some("password")).unwrap();
///     client.update_access_token(access_token);
/// ```
#[derive(Debug)]
pub struct Basic([(&'static str, Box<str>); 1]);

impl Basic {
    /// Create `Basic` access token.
    #[inline]
    pub fn create(username: &str, password: Option<&str>) -> CometdResult<Self> {
        let capacity = encoded_len(BASIC.len(), true)
            .unwrap_or_default()
            .saturating_add(encoded_len(username.len(), true).unwrap_or_default())
            .saturating_add(password.map_or(0, str::len));

        let mut basic = Vec::with_capacity(capacity);
        basic.extend_from_slice(BASIC);

        let mut base64_writer = Base64Writer::new(&mut basic, &STANDARD);
        write!(base64_writer, "{username}:").map_err(CometdError::unexpected)?;
        if let Some(password) = password {
            write!(base64_writer, "{password}").map_err(CometdError::unexpected)?;
        }
        drop(base64_writer);

        Ok(Self([(
            AUTHORIZATION.as_str(),
            String::from_utf8(basic)
                .map_err(CometdError::unexpected)?
                .into_boxed_str(),
        )]))
    }
}

impl AccessToken for Basic {
    fn get_authorization_header(&self) -> &[(&'static str, Box<str>)] {
        &self.0
    }
}
