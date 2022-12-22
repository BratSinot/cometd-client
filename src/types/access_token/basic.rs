use crate::types::{AccessToken, CometdError, CometdResult};
use base64::{engine::DEFAULT_ENGINE, write::EncoderWriter as Base64Writer};
use hyper::header::AUTHORIZATION;
use std::io::Write;

const BASIC: &[u8] = b"Basic ";

/// `Basic` can be used as `AccessToken` for basic authorization ('authorization: Basic VmFzeWE6UGV0eWE=').
///
/// # Example
/// ```rust
/// # use cometd_client::{types::access_token::Basic, CometdClientBuilder};
/// # let client = CometdClientBuilder::new("http://[::1]:1025/".parse().unwrap()).build().unwrap();
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
        let capacity = calculate_padded_base64_len(BASIC.len())
            .saturating_add(calculate_padded_base64_len(username.len()))
            .saturating_add(password.map(str::len).unwrap_or(0));

        let mut basic = Vec::with_capacity(capacity);
        basic.extend_from_slice(BASIC);

        let mut base64_writer = Base64Writer::from(&mut basic, &DEFAULT_ENGINE);
        write!(base64_writer, "{username}:").map_err(CometdError::unexpected_error)?;
        if let Some(password) = password {
            write!(base64_writer, "{password}").map_err(CometdError::unexpected_error)?;
        }
        drop(base64_writer);

        Ok(Self([(
            AUTHORIZATION.as_str(),
            String::from_utf8(basic)
                .map_err(CometdError::unexpected_error)?
                .into_boxed_str(),
        )]))
    }
}

impl AccessToken for Basic {
    fn get_authorization_header<'a>(&'a self) -> &[(&'static str, Box<str>)] {
        &self.0
    }
}

#[inline]
const fn calculate_padded_base64_len(len: usize) -> usize {
    // ((4 * n / 3) + 3) & !3
    4usize
        .saturating_mul(len)
        .saturating_div(3)
        .saturating_add(3)
        & !3
}
