mod builder;
mod connect;
mod disconnect;
mod handshake;
mod subscribe;

pub use builder::*;

use crate::{types::AccessToken, ArcSwapOptionExt};
use arc_swap::ArcSwapOption;
use hyper::{
    client::HttpConnector, header::SET_COOKIE, http::HeaderValue, Body, Client, Response, Uri,
};
use std::sync::atomic::{AtomicUsize, Ordering};

/// A cometd Client.
#[derive(Debug)]
pub struct CometdClient {
    handshake_endpoint: Uri,
    subscribe_endpoint: Uri,
    connect_endpoint: Uri,
    disconnect_endpoint: Uri,
    timeout_ms: u64,
    interval_ms: u64,

    id: AtomicUsize,
    pub(crate) access_token: ArcSwapOption<Box<dyn AccessToken>>,
    pub(crate) cookie: ArcSwapOption<HeaderValue>,
    client_id: ArcSwapOption<Box<str>>,
    pub(crate) http_client: Client<HttpConnector>,
}

impl CometdClient {
    /// Method for update access token.
    ///
    /// # Example
    /// ```rust
    /// # use cometd_client::{types::access_token::Basic, CometdClientBuilder};
    /// # let client = CometdClientBuilder::new("http://[::1]:1025/".parse().unwrap()).build().unwrap();
    ///
    ///     let access_token = Basic::create("username", Some("password")).unwrap();
    ///     client.update_access_token(access_token);
    /// ```
    #[inline(always)]
    pub fn update_access_token<AT>(&self, access_token: AT)
    where
        AT: AccessToken + 'static,
    {
        self.access_token.store_value(Box::new(access_token));
    }

    #[inline(always)]
    pub(crate) fn next_id(&self) -> String {
        self.id.fetch_add(1, Ordering::Relaxed).to_string()
    }

    #[inline(always)]
    pub(crate) async fn extract_and_store_cookie(&self, response: &mut Response<Body>) {
        if let Some(cookie) = response.headers_mut().remove(SET_COOKIE) {
            self.cookie.store_value(cookie);
        }
    }
}
