#[cfg(feature = "basic")]
mod basic;
mod bearer;

#[cfg(feature = "basic")]
pub use basic::*;
pub use bearer::*;

use core::fmt::Debug;

/// Trait which can be used for implementing custom access token.
///
/// # Example:
/// ```rust,no_run
/// # use cometd_client::types::AccessToken;
///     #[derive(Debug)]
///     struct SuperToken(Box<str>);
///
///     impl SuperToken {
///         pub fn new() -> Self {
///             Self("super-name Jindřich".into())
///         }
///     }
///
///     impl AccessToken for SuperToken {
///         fn get_authorization_token(&self) -> &str {
///             &self.0
///         }
///     }
/// ```
pub trait AccessToken: Debug + Sync + Send + 'static {
    /// Return reference to array of pairs `(<HeaderName>, <HeaderValue>)`.
    fn get_authorization_token(&self) -> &str;
}
