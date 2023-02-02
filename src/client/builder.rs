mod client_task;

use crate::client::CometdClientInner;
use crate::{
    consts::{
        DEFAULT_COMMAND_CHANNEL_CAPACITY, DEFAULT_EVENT_CHANNEL_CAPACITY, DEFAULT_INTERVAL_MS,
        DEFAULT_TIMEOUT_MS,
    },
    ext::CookieJarExt,
    types::{AccessToken, CometdResult},
    CometdClient,
};
use arc_swap::ArcSwapOption;
use async_broadcast::broadcast;
use cookie::{Cookie, CookieJar};
use hyper::Client;
use serde::de::DeserializeOwned;
use std::borrow::Cow;
use tokio::sync::{mpsc, RwLock};
use url::Url;

/// A builder to construct `CometdClient`.
#[derive(Debug)]
pub struct CometdClientBuilder<'a, 'b, 'c, 'd, 'e> {
    endpoint: &'a Url,
    handshake_base_path: &'b str,
    subscribe_base_path: &'c str,
    connect_base_path: &'d str,
    disconnect_base_path: &'e str,
    timeout_ms: Option<u64>,
    interval_ms: Option<u64>,
    access_token: Option<Box<dyn AccessToken>>,
    cookies: Option<CookieJar>,
    commands_channel_capacity: usize,
    events_channel_capacity: usize,
}

impl<'a, 'b, 'c, 'd, 'e> CometdClientBuilder<'a, 'b, 'c, 'd, 'e> {
    /// Construct a new `ClientBuilder`.
    #[inline(always)]
    pub fn new(endpoint: &'a Url) -> Self {
        Self {
            endpoint,
            handshake_base_path: "",
            subscribe_base_path: "",
            connect_base_path: "",
            disconnect_base_path: "",
            timeout_ms: None,
            interval_ms: None,
            access_token: None,
            cookies: None,
            commands_channel_capacity: DEFAULT_COMMAND_CHANNEL_CAPACITY,
            events_channel_capacity: DEFAULT_EVENT_CHANNEL_CAPACITY,
        }
    }

    /// Return a `CometdClient`.
    ///
    /// # Example
    /// ```rust
    /// # use cometd_client::{CometdClient, CometdClientBuilder};
    /// # let _ = || -> cometd_client::types::CometdResult<_> {
    /// # #[derive(serde::Deserialize)]
    /// # struct Data { msg: String, }
    /// let client = CometdClientBuilder::new(&"http://[::1]:1025/notifications/".parse()?)
    ///     .build()?;
    /// # let client: CometdClient<Data> = client;
    /// # Ok(()) };
    /// ```
    #[inline(always)]
    pub fn build<Msg>(self) -> CometdResult<CometdClient<Msg>>
    where
        Msg: DeserializeOwned + Send + Sync + 'static,
    {
        let Self {
            endpoint: base_url,
            handshake_base_path,
            subscribe_base_path,
            connect_base_path,
            disconnect_base_path,
            timeout_ms,
            interval_ms,
            access_token,
            cookies,
            commands_channel_capacity,
            events_channel_capacity,
        } = self;

        let handshake_endpoint =
            String::from(base_url.join(handshake_base_path)?.join("handshake")?).try_into()?;
        let subscribe_endpoint = String::from(base_url.join(subscribe_base_path)?).try_into()?;
        let connect_endpoint =
            String::from(base_url.join(connect_base_path)?.join("connect")?).try_into()?;
        let disconnect_endpoint =
            String::from(base_url.join(disconnect_base_path)?.join("disconnect")?).try_into()?;
        let timeout_ms = timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS);
        let interval_ms = interval_ms.unwrap_or(DEFAULT_INTERVAL_MS);
        let id = Default::default();
        let access_token = access_token
            .map(ArcSwapOption::from_pointee)
            .unwrap_or_default();
        let cookies_string_cache = cookies
            .as_ref()
            .map(CookieJarExt::make_string)
            .map(ArcSwapOption::from_pointee)
            .unwrap_or_default();
        let cookies = cookies.unwrap_or_default();
        let client_id = Default::default();
        let http_client = Client::builder().build_http();

        let (cmd_tx, cmd_rx) = mpsc::channel(commands_channel_capacity);
        let (event_tx, mut event_rx) = broadcast(events_channel_capacity);
        event_rx.set_await_active(false);

        let inner = CometdClientInner {
            handshake_endpoint,
            subscribe_endpoint,
            connect_endpoint,
            disconnect_endpoint,
            timeout_ms,
            interval_ms,
            id,
            access_token,
            cookies: RwLock::new(cookies),
            cookies_string_cache,
            client_id,
            http_client,
        };

        client_task::spawn(inner, cmd_rx, event_tx);

        Ok(CometdClient {
            cmd_tx,
            inactive_event_rx: event_rx.deactivate(),
        })
    }

    /// Set cometd server handshake url path.
    ///
    /// # Example
    /// ```rust
    /// # use cometd_client::{CometdClient, CometdClientBuilder};
    /// # let _ = || -> cometd_client::types::CometdResult<_> {
    /// # #[derive(serde::Deserialize)]
    /// # struct Data { msg: String, }
    ///
    ///     let app = CometdClientBuilder::new(&"http://[::1]:1025/notifications/".parse()?)
    ///         .handshake_base_path("hand/") // http://[::1]:1025/notifications/hand/handshake
    ///         .build()?;
    /// # let app: CometdClient<Data> = app;
    /// # Ok(()) };
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn handshake_base_path(mut self, url: &'b str) -> Self {
        self.handshake_base_path = url;
        self
    }

    /// Set cometd server subscribe url path.
    ///
    /// # Example
    /// ```rust
    /// # use cometd_client::{CometdClient, CometdClientBuilder};
    /// # let _ = || -> cometd_client::types::CometdResult<_> {
    /// # #[derive(serde::Deserialize)]
    /// # struct Data { msg: String, }
    ///
    ///     let app = CometdClientBuilder::new(&"http://[::1]:1025/notifications/".parse()?)
    ///         .subscribe_base_path("sub/") // http://[::1]:1025/notifications/sub/
    ///         .build()?;
    /// # let app: CometdClient<Data> = app;
    /// # Ok(()) };
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn subscribe_base_path(mut self, url: &'c str) -> Self {
        self.subscribe_base_path = url;
        self
    }

    /// Set cometd server connect url path.
    ///
    /// # Example
    /// ```rust
    /// # use cometd_client::{CometdClient, CometdClientBuilder};
    /// # let _ = || -> cometd_client::types::CometdResult<_> {
    /// # #[derive(serde::Deserialize)]
    /// # struct Data { msg: String, }
    ///     let app = CometdClientBuilder::new(&"http://[::1]:1025/notifications/".parse()?)
    ///         .connect_base_path("con/") // http://[::1]:1025/notifications/con/connect
    ///         .build()?;
    /// # let app: CometdClient<Data> = app;
    /// # Ok(()) };
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn connect_base_path(mut self, url: &'d str) -> Self {
        self.connect_base_path = url;
        self
    }

    /// Set cometd server disconnect url path.
    ///
    /// # Example
    /// ```rust
    /// # use cometd_client::{CometdClient, CometdClientBuilder};
    /// # let _ = || -> cometd_client::types::CometdResult<_> {
    /// # #[derive(serde::Deserialize)]
    /// # struct Data { msg: String, }
    ///     let app = CometdClientBuilder::new(&"http://[::1]:1025/notifications/".parse()?)
    ///         .disconnect_base_path("con/") // http://[::1]:1025/notifications/discon/disconnect
    ///         .build()?;
    /// # let app: CometdClient<Data> = app;
    /// # Ok(()) };
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn disconnect_base_path(mut self, url: &'e str) -> Self {
        self.disconnect_base_path = url;
        self
    }

    /// Set `timeout` option in handshake request.
    #[inline(always)]
    #[must_use]
    pub const fn timeout_ms(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = Some(timeout_ms);
        self
    }

    /// Set `interval` option in handshake request.
    #[inline(always)]
    #[must_use]
    pub const fn interval_ms(mut self, interval_ms: u64) -> Self {
        self.interval_ms = Some(interval_ms);
        self
    }

    /// Set `interval` option in handshake request.
    #[inline(always)]
    #[must_use]
    pub fn access_token<AT>(self, access_token: AT) -> Self
    where
        AT: AccessToken + 'static,
    {
        Self {
            access_token: Some(Box::new(access_token)),
            ..self
        }
    }

    /// Set `cookie`.
    #[inline(always)]
    #[must_use]
    pub fn cookie<N, V>(self, name: N, value: V) -> Self
    where
        N: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        self.cookies([(name, value)])
    }

    /// Set `cookies`.
    #[inline(always)]
    #[must_use]
    pub fn cookies<N, V>(self, cookies: impl IntoIterator<Item = (N, V)>) -> Self
    where
        N: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        let mut cookie_jar = CookieJar::new();

        for (name, value) in cookies {
            cookie_jar.add(Cookie::new(name, value))
        }

        Self {
            cookies: Some(cookie_jar),
            ..self
        }
    }

    /// Set capacity of `Event` channel.
    #[inline(always)]
    #[must_use]
    pub const fn events_channel_capacity(mut self, events_channel_capacity: usize) -> Self {
        self.events_channel_capacity = events_channel_capacity;
        self
    }

    /// Set capacity of internal commands channel.
    #[inline(always)]
    #[must_use]
    pub const fn commands_channel_capacity(mut self, commands_channel_capacity: usize) -> Self {
        self.commands_channel_capacity = commands_channel_capacity;
        self
    }
}
