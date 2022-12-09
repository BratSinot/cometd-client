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
    #[error("Error during handshake request: `{0}`.")]
    HandshakeError(Box<dyn Error>),
    #[error("Error during subscribe request: `{0}`.")]
    SubscribeError(Box<dyn Error>),
    #[error("Error during connect request: `{0}`.")]
    ConnectError(Box<dyn Error>),
    #[error("Error during disconnect request: `{0}`.")]
    DisconnectError(Box<dyn Error>),

    #[error("Got unexpected error: `{0}`")]
    UnexpectedError(Box<dyn Error>),
}

impl CometdError {
    #[inline(always)]
    pub(crate) fn unexpected_error<E: Error + 'static>(err: E) -> Self {
        Self::UnexpectedError(err.into())
    }

    #[inline(always)]
    pub(crate) fn handshake_error<E: Error + 'static>(err: E) -> Self {
        Self::HandshakeError(err.into())
    }

    #[inline(always)]
    pub(crate) fn subscribe_error<E: Error + 'static>(err: E) -> Self {
        Self::SubscribeError(err.into())
    }

    #[inline(always)]
    pub(crate) fn connect_error<E: Error + 'static>(err: E) -> Self {
        Self::ConnectError(err.into())
    }

    #[inline(always)]
    pub(crate) fn disconnect_error<E: Error + 'static>(err: E) -> Self {
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
