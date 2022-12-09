#[cfg(feature = "basic")]
mod basic;
mod bearer;

#[cfg(feature = "basic")]
pub use basic::*;
pub use bearer::*;
use std::fmt::Debug;

/// Trait which can be used for implementing custom access token.
///
/// # Example:
/// ```rust
/// # use cometd_client::AccessToken;
///     #[derive(Debug)]
///     struct SuperToken([(&'static str, Box<str>); 2]);
///
///     impl SuperToken {
///         pub fn new() -> Self {
///             Self([("super-name", "Jindřich".into()), ("super-city", "Skalica".into())])
///         }
///     }
///
///     impl AccessToken for SuperToken {
///         fn get_authorization_header<'a>(&'a self) -> &[(&'static str, Box<str>)] {
///             &self.0
///         }
///     }
/// ```
pub trait AccessToken: Debug {
    /// Return reference to array of pairs `(<HeaderName>, <HeaderValue>)`.
    fn get_authorization_header<'a>(&'a self) -> &[(&'static str, Box<str>)];
}
