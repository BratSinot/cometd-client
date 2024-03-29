use crate::types::{AccessToken, CometdError, CometdResult};
use base64::{
    encoded_len, engine::general_purpose::STANDARD, write::EncoderWriter as Base64Writer,
};
use core::fmt::{Debug, Formatter};
use std::io::Write;

const BASIC: &[u8] = b"Basic ";

/// `Basic` can be used as `AccessToken` for basic authorization ('authorization: Basic VmFzeWE6UGV0eWE=').
///
/// # Example
/// ```rust,no_run
/// # use cometd_client::{types::access_token::Basic, CometdClientBuilder};
///
/// # async {
///     let access_token = Basic::create("username", Some("password"))?;
///
///     let client = CometdClientBuilder::new(&"http://[::1]:1025/".parse()?)
///         .access_token(access_token)
///         .build::<()>()?;
/// # Result::<_, Box<dyn std::error::Error>>::Ok(())
/// # };
/// ```
pub struct Basic(Box<str>);

impl Debug for Basic {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Basic(****)")
    }
}

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

        let ret = String::from_utf8(basic)
            .map_err(CometdError::unexpected)?
            .into_boxed_str();

        Ok(Self(ret))
    }
}

impl AccessToken for Basic {
    fn get_authorization_token(&self) -> &str {
        &self.0
    }
}
