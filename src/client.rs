mod builder;
mod connect;
mod disconnect;
mod handshake;
mod subscribe;

pub use builder::*;

use crate::{ext::CookieJarExt, types::*, ArcSwapOptionExt};
use arc_swap::ArcSwapOption;
use cookie::{Cookie, CookieJar};
use core::sync::atomic::{AtomicUsize, Ordering};
use hyper::{client::HttpConnector, header::SET_COOKIE, http::HeaderValue, Client, HeaderMap, Uri};
use serde::Serialize;
use serde_json::json;
use tokio::sync::RwLock;

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
    timeout_ms: u64,
    interval_ms: u64,
    number_of_retries: usize,

    id: AtomicUsize,
    pub(crate) access_token: ArcSwapOption<Box<dyn AccessToken>>,
    cookies: RwLock<CookieJar>,
    pub(crate) cookies_string_cache: ArcSwapOption<Box<str>>,
    client_id: ArcSwapOption<Box<str>>,
    pub(crate) http_client: Client<HttpConnector>,
}

impl<Msg> CometdClient<Msg> {
    /// Return client event receiver channel.
    ///
    /// # Example
    /// ```rust
    /// # use cometd_client::{CometdClientBuilder, types::CometdResult};
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// #   let client = CometdClientBuilder::new(&"http://[::1]:1025/".parse().unwrap()).build::<()>().unwrap();
    ///     let mut event_rx = client.rx();
    ///     
    ///     client.subscribe(&["/topic0"]).await;
    ///
    ///     while let Ok(event) = event_rx.recv().await {
    ///         println!("Got cometd client event: `{event:?}`.");
    ///     }
    /// # }
    /// ```
    #[inline(always)]
    pub fn rx(&self) -> EventReceiver<Msg> {
        self.inactive_event_rx.activate_cloned()
    }

    /// Ask client command loop to send subscribe request.
    ///
    /// # Example
    /// ```rust
    /// # use cometd_client::{CometdClientBuilder, types::CometdResult};
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
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
    pub(crate) async fn extract_and_store_cookie(&self, headers: &HeaderMap) {
        let mut redo_cache = false;

        let mut cookies = self.cookies.write().await;
        for cookie in headers
            .get_all(SET_COOKIE)
            .iter()
            .map(HeaderValue::to_str)
            .filter_map(Result::ok)
            .map(str::to_string)
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
