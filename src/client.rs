mod builder;
mod connect;
mod disconnect;
mod handshake;
mod subscribe;

pub use builder::*;

use crate::{ext::CookieJarExt as _, types::*, ArcSwapOptionExt};
use arc_swap::ArcSwapOption;
use cookie::{Cookie, CookieJar};
use core::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};
use hyper::{client::HttpConnector, header::SET_COOKIE, http::HeaderValue, Client, HeaderMap, Uri};
use serde::Serialize;
use serde_json::json;
use std::sync::{Mutex, PoisonError};

/// A cometd Client.
#[derive(Debug)]
pub struct CometdClient<Msg> {
    cmd_tx: CmdSender,
    inactive_event_rx: InactiveEventReceiver<Msg>,
}

#[derive(Debug)]
pub(crate) struct CometdClientInner {
    handshake_endpoint: Uri,
    subscribe_endpoint: Uri,
    connect_endpoint: Uri,
    disconnect_endpoint: Uri,
    timeout: Duration,
    interval: Duration,
    number_of_retries: usize,

    id: AtomicUsize,
    pub(crate) access_token: ArcSwapOption<Box<dyn AccessToken>>,
    cookies: Mutex<CookieJar>,
    pub(crate) cookies_string_cache: ArcSwapOption<Box<str>>,
    client_id: ArcSwapOption<Box<str>>,
    pub(crate) http_client: Client<HttpConnector>,
    pub(crate) request_timeout: Duration,
}

impl<Msg> CometdClient<Msg> {
    /// Return client event receiver channel.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use cometd_client::{CometdClientBuilder, types::CometdResult};
    /// # async fn _fun() {
    /// #   let client = CometdClientBuilder::new(&"http://[::1]:1025/".parse().unwrap()).build::<()>().unwrap();
    ///     let mut event_rx = client.rx();
    ///     
    ///     client.subscribe(&["/topic0"]).await;
    ///
    ///     while let Some(event) = event_rx.recv().await {
    ///         println!("Got cometd client event: `{event:?}`.");
    ///     }
    /// # }
    /// ```
    #[inline(always)]
    pub fn rx(&self) -> CometdEventReceiver<Msg> {
        CometdEventReceiver(self.inactive_event_rx.activate_cloned())
    }

    /// Ask client command loop to send subscribe request.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use cometd_client::{CometdClientBuilder, types::CometdResult};
    /// # async fn _fun() {
    /// #   let client = CometdClientBuilder::new(&"http://[::1]:1025/".parse().unwrap()).build::<()>().unwrap();
    ///     client.subscribe(&["/topic0", "/topic1"]).await;
    /// # }
    /// ```
    #[inline(always)]
    pub async fn subscribe(&self, subscriptions: &[impl Serialize + Send + Sync]) {
        let _ = self
            .cmd_tx
            .send(Command::Subscribe(json!(subscriptions)))
            .await;
    }
}

impl CometdClientInner {
    #[inline(always)]
    pub(crate) fn next_id(&self) -> String {
        self.id.fetch_add(1, Ordering::Relaxed).to_string()
    }

    #[inline]
    pub(crate) fn extract_and_store_cookie(&self, headers: &HeaderMap) {
        let mut redo_cache = false;

        let mut cookies = self.cookies.lock().unwrap_or_else(PoisonError::into_inner);
        for cookie in headers
            .get_all(SET_COOKIE)
            .into_iter()
            .map(HeaderValue::to_str)
            .filter_map(Result::ok)
            .map(str::to_owned)
            .map(Cookie::parse)
            .filter_map(Result::ok)
        {
            cookies.add(cookie);
            redo_cache = true;
        }

        if redo_cache {
            self.cookies_string_cache.store_value(cookies.make_string());
        }
    }
}
