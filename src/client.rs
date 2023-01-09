mod builder;
mod connect;
mod disconnect;
mod handshake;
mod subscribe;

pub use builder::*;

use crate::{types::AccessToken, ArcSwapOptionExt};
use arc_swap::ArcSwapOption;
use reqwest::Client;
use std::sync::atomic::{AtomicUsize, Ordering};
use url::Url;

/// A cometd Client.
#[derive(Debug)]
pub struct CometdClient {
    handshake_endpoint: Url,
    subscribe_endpoint: Url,
    connect_endpoint: Url,
    disconnect_endpoint: Url,
    timeout_ms: u64,
    interval_ms: u64,

    id: AtomicUsize,
    pub(crate) access_token: ArcSwapOption<Box<dyn AccessToken>>,
    client_id: ArcSwapOption<Box<str>>,
    pub(crate) http_client: Client,
}

impl CometdClient {
    /// Method for update access token.
    ///
    /// # Example
    /// ```rust
    /// # use cometd_client::{types::access_token::Basic, CometdClientBuilder};
    /// # let client = CometdClientBuilder::new(&"http://[::1]:1025/".parse().unwrap()).build().unwrap();
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
}
