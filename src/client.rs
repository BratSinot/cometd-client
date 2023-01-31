mod builder;
mod command_listener;
mod connect;
mod disconnect;
mod handshake;
mod subscribe;

pub use builder::*;

use crate::{
    ext::CookieJarExt,
    types::{AccessToken, Command, Event},
    ArcSwapOptionExt,
};
use arc_swap::ArcSwapOption;
use async_broadcast::{InactiveReceiver, Receiver as MpmcReceiver};
use cookie::{Cookie, CookieJar};
use core::sync::atomic::{AtomicUsize, Ordering};
use hyper::{client::HttpConnector, header::SET_COOKIE, http::HeaderValue, Client, HeaderMap, Uri};
use std::{borrow::Cow, sync::Arc};
use tokio::sync::{mpsc::Sender as MpscSender, RwLock};

/// A cometd Client.
#[derive(Debug)]
pub struct CometdClient(pub(crate) Arc<Inner>);

#[derive(Debug)]
pub(crate) struct Inner {
    handshake_endpoint: Uri,
    subscribe_endpoint: Uri,
    connect_endpoint: Uri,
    disconnect_endpoint: Uri,
    timeout_ms: u64,
    interval_ms: u64,

    id: AtomicUsize,
    pub(crate) access_token: ArcSwapOption<Box<dyn AccessToken>>,
    cookies: RwLock<CookieJar>,
    pub(crate) cookies_string_cache: ArcSwapOption<Box<str>>,
    client_id: ArcSwapOption<Box<str>>,
    pub(crate) http_client: Client<HttpConnector>,

    commands_tx: MpscSender<Command>,
    inactive_event_rx: InactiveReceiver<Arc<Event>>,
}

impl CometdClient {
    /// Method for update access token.
    ///
    /// # Example
    /// ```rust
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// # use cometd_client::{types::access_token::Basic, CometdClientBuilder};
    /// # let client = CometdClientBuilder::new(&"http://[::1]:1025/".parse().unwrap()).build().unwrap();
    ///
    ///     let access_token = Basic::create("username", Some("password")).unwrap();
    ///     client.update_access_token(access_token);
    /// # }
    /// ```
    #[inline(always)]
    pub fn update_access_token<AT>(&self, access_token: AT)
    where
        AT: AccessToken + 'static,
    {
        self.0.access_token.store_value(Box::new(access_token));
    }

    /// Method for adding cookies.
    ///
    /// # Example
    /// ```rust
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// # use cometd_client::{types::access_token::Basic, CometdClientBuilder};
    /// # let client = CometdClientBuilder::new(&"http://[::1]:1025/".parse().unwrap()).build().unwrap();
    ///
    ///     client.add_cookies([("a", "1")]);
    /// # }
    /// ```
    #[inline(always)]
    pub async fn add_cookies<N, V>(&self, cookies: impl IntoIterator<Item = (N, V)> + Send)
    where
        N: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        let mut cookie_jar = self.0.cookies.write().await;
        for (name, value) in cookies {
            cookie_jar.add(Cookie::new(name, value));
        }

        self.0
            .cookies_string_cache
            .store_value(cookie_jar.make_string());
    }

    /// Method for getting new `Event` receiver.
    ///
    /// # Example
    /// ```rust
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// # use cometd_client::{types::access_token::Basic, CometdClientBuilder};
    /// # let client = CometdClientBuilder::new(&"http://[::1]:1025/".parse().unwrap()).build().unwrap();
    /// # async {
    ///     let mut rx = client.rx();
    ///     while let Ok(event) = rx.recv().await {
    ///         println!("Got event: `{event:?}`.");
    ///     }
    /// # };
    /// # }
    /// ```
    #[inline(always)]
    pub fn rx(&self) -> MpmcReceiver<Arc<Event>> {
        self.0.inactive_event_rx.activate_cloned()
    }
}

impl Inner {
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
