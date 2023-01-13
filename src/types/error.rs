// https://github.com/rust-lang/rust-clippy/issues/10198
#![allow(clippy::std_instead_of_core)]

use crate::types::Reconnect;
use hyper::http::uri::InvalidUri;
use std::{borrow::Cow, error::Error};
use url::ParseError as UrlParseError;

#[allow(missing_docs)]
pub type CometdResult<T> = Result<T, CometdError>;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum CometdError {
    #[error("Endpoint wasn't set in builder.")]
    MissingEndpoint,
    #[error("Url parse error: `{0}`.")]
    InvalidUrl(#[from] UrlParseError),
    #[error("Url parse error: `{0}`.")]
    InvalidUri(#[from] InvalidUri),
    #[error("Error during handshake request: `{1}`.")]
    HandshakeError(Reconnect, Box<dyn Error + Sync + Send + 'static>),
    #[error("Error during subscribe request: `{1}`.")]
    SubscribeError(Reconnect, Box<dyn Error + Sync + Send + 'static>),
    #[error("Error during connect request: `{1}`.")]
    ConnectError(Reconnect, Box<dyn Error + Sync + Send + 'static>),
    #[error("Error during disconnect request: `{0}`.")]
    DisconnectError(Box<dyn Error + Sync + Send + 'static>),

    #[error("Got unexpected error: `{0}`")]
    UnexpectedError(Box<dyn Error + Sync + Send + 'static>),
}

impl CometdError {
    /// Return advice if server set it in response.
    #[inline]
    pub const fn advice(&self) -> Reconnect {
        match *self {
            CometdError::MissingEndpoint
            | CometdError::InvalidUrl(_)
            | CometdError::InvalidUri(_)
            | CometdError::DisconnectError(_)
            | CometdError::UnexpectedError(_) => Reconnect::None,
            CometdError::HandshakeError(advice, _)
            | CometdError::SubscribeError(advice, _)
            | CometdError::ConnectError(advice, _) => advice,
        }
    }
}

impl CometdError {
    #[inline(always)]
    pub(crate) fn unexpected_error<E: Error + Sync + Send + 'static>(err: E) -> Self {
        Self::UnexpectedError(err.into())
    }

    #[inline(always)]
    pub(crate) fn handshake_error<E: Error + Sync + Send + 'static>(
        advice: Option<Reconnect>,
        err: E,
    ) -> Self {
        Self::HandshakeError(advice.unwrap_or_default(), err.into())
    }

    #[inline(always)]
    pub(crate) fn subscribe_error<E: Error + Sync + Send + 'static>(
        advice: Option<Reconnect>,
        err: E,
    ) -> Self {
        Self::SubscribeError(advice.unwrap_or_default(), err.into())
    }

    #[inline(always)]
    pub(crate) fn connect_error<E: Error + Sync + Send + 'static>(
        advice: Option<Reconnect>,
        err: E,
    ) -> Self {
        Self::ConnectError(advice.unwrap_or_default(), err.into())
    }

    #[inline(always)]
    pub(crate) fn disconnect_error<E: Error + Sync + Send + 'static>(err: E) -> Self {
        Self::DisconnectError(err.into())
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum InnerError {
    #[error("{0}")]
    WrongResponse(Cow<'static, str>),
    #[error("Make handshake before request.")]
    MissingClientId,
}
