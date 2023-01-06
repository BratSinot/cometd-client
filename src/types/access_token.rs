#[cfg(feature = "basic")]
mod basic;
mod bearer;

#[cfg(feature = "basic")]
pub use basic::*;
pub use bearer::*;
pub use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

use std::fmt::Debug;

/// Trait which can be used for implementing custom access token.
///
/// # Example:
/// ```rust
/// # use cometd_client::types::access_token::*;
///     #[derive(Debug)]
///     struct SuperToken(HeaderMap);
///
///     impl SuperToken {
///         pub fn new() -> Self {
///             Self(HeaderMap::from_iter([
///                 (
///                     HeaderName::from_static("super-name"),
///                     HeaderValue::from_static("Jindřich"),
///                 ),
///                 (
///                     HeaderName::from_static("super-city"),
///                     HeaderValue::from_static("Skalica"),
///                 ),
///             ]))
///         }
///     }
///
///     impl AccessToken for SuperToken {
///         fn get_authorization_header(&self) -> HeaderMap {
///             self.0.clone()
///         }
///     }
/// ```
pub trait AccessToken: Debug + Sync + Send + 'static {
    /// Return reference to array of pairs `(<HeaderName>, <HeaderValue>)`.
    fn get_authorization_header(&self) -> HeaderMap;
}
