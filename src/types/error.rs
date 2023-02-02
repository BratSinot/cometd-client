// https://github.com/rust-lang/rust-clippy/issues/10198
#![allow(clippy::std_instead_of_core)]

use crate::types::Reconnect;
use hyper::{http::uri::InvalidUri, Error as HyperError, StatusCode};
use serde_json::Error as JsonError;
use std::{borrow::Cow, error::Error};
use url::ParseError as UrlParseError;

#[allow(missing_docs)]
pub type CometdResult<T> = Result<T, CometdError>;

#[allow(missing_docs)]
#[derive(Debug, Copy, Clone)]
pub enum ErrorKind {
    Handshake,
    Subscribe,
    Connect,
    Disconnect,
}

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum CometdError {
    #[error("Endpoint wasn't set in builder.")]
    MissingEndpoint,
    #[error("Url parse error: `{0}`.")]
    InvalidUrl(#[from] UrlParseError),
    #[error("Url parse error: `{0}`.")]
    InvalidUri(#[from] InvalidUri),
    #[error("Got request error at {0:?}: `{1}`.")]
    Request(ErrorKind, HyperError),
    /// Return if status code non ok (in range [200, 300)).
    /// Body will be empty if got error while fetching body.
    #[error("Got unsuccessful StatusCode error at {0:?}: `{1}`.")]
    StatusCode(ErrorKind, StatusCode, Vec<u8>),
    #[error("Got fetching body error at {0:?}: `{1}`.")]
    FetchBody(ErrorKind, HyperError),
    #[error("Got parsing body error at {0:?}: `{1}`.")]
    ParseBody(ErrorKind, JsonError),
    #[error("Got wring response at {0:?}: `{2}`")]
    WrongResponse(ErrorKind, Reconnect, Cow<'static, str>),
    #[error("Make handshake before {0:?} request.")]
    MissingClientId(ErrorKind),
    #[error("Got unexpected error: `{0}`")]
    Unexpected(Box<dyn Error + Sync + Send + 'static>),
    #[error("Got unexpected internal error: `{0}`")]
    Internal(&'static str),
}

impl CometdError {
    #[inline(always)]
    pub(crate) fn wrong_response<E>(kind: ErrorKind, advice: Reconnect, error_message: E) -> Self
    where
        Cow<'static, str>: From<E>,
    {
        Self::WrongResponse(kind, advice, Cow::from(error_message))
    }

    #[inline(always)]
    pub(crate) fn unexpected<E: Error + Sync + Send + 'static>(error: E) -> Self {
        Self::Unexpected(Box::from(error))
    }
}
