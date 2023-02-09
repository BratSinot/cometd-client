use crate::types::AccessToken;
use core::fmt::{Debug, Display, Formatter};

/// `Bearer` can be used as `AccessToken` for bearer authorization ('authorization: Bearer f0596451-af4d-40f4-a290-b5e8372c110b').
///
/// # Example
/// ```rust,no_run
/// # use cometd_client::{types::access_token::Bearer, CometdClientBuilder};
///
/// # async {
///     let access_token = Bearer::new("f0596451-af4d-40f4-a290-b5e8372c110b");
///
///     let client = CometdClientBuilder::new(&"http://[::1]:1025/".parse()?)
///         .access_token(access_token)
///         .build::<()>()?;
/// # Result::<_, Box<dyn std::error::Error>>::Ok(())
/// # };
/// ```
pub struct Bearer(Box<str>);

impl Debug for Bearer {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Bearer(****)")
    }
}

impl Bearer {
    /// Create `Bearer` access token.
    #[inline(always)]
    pub fn new<T: Display>(token: T) -> Self {
        Self(format!("Bearer {token}").into_boxed_str())
    }
}

impl AccessToken for Bearer {
    fn get_authorization_token(&self) -> &str {
        &self.0
    }
}
