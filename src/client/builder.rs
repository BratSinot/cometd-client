use crate::{
    consts::{DEFAULT_INTERVAL_MS, DEFAULT_TIMEOUT_MS},
    types::{AccessToken, CometdResult},
    CometdClient,
};
use arc_swap::ArcSwapOption;
use reqwest::Client;
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
        }
    }

    /// Return a `CometdClient`.
    ///
    /// # Example
    /// ```rust
    /// use cometd_client::CometdClientBuilder;
    ///
    /// # let _ = || -> cometd_client::types::CometdResult<_> {
    /// let client = CometdClientBuilder::new(&"http://[::1]:1025/notifications/".parse()?)
    ///     .build()?;
    /// # Ok(()) };
    /// ```
    #[inline(always)]
    pub fn build(self) -> CometdResult<CometdClient> {
        let Self {
            endpoint: base_url,
            handshake_base_path,
            subscribe_base_path,
            connect_base_path,
            disconnect_base_path,
            timeout_ms,
            interval_ms,
            access_token,
        } = self;

        let handshake_endpoint = base_url.join(handshake_base_path)?.join("handshake")?;
        let subscribe_endpoint = base_url.join(subscribe_base_path)?;
        let connect_endpoint = base_url.join(connect_base_path)?.join("connect")?;
        let disconnect_endpoint = base_url.join(disconnect_base_path)?.join("disconnect")?;
        let timeout_ms = timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS);
        let interval_ms = interval_ms.unwrap_or(DEFAULT_INTERVAL_MS);
        let id = Default::default();
        let access_token = access_token
            .map(ArcSwapOption::from_pointee)
            .unwrap_or_default();
        let client_id = Default::default();
        let http_client = Client::new();

        Ok(CometdClient {
            handshake_endpoint,
            subscribe_endpoint,
            connect_endpoint,
            disconnect_endpoint,
            timeout_ms,
            interval_ms,
            id,
            access_token,
            client_id,
            http_client,
        })
    }

    /// Set cometd server handshake url path.
    ///
    /// # Example
    /// ```rust
    /// use cometd_client::CometdClientBuilder;
    ///
    /// # let _ = || -> cometd_client::types::CometdResult<_> {
    /// let app = CometdClientBuilder::new(&"http://[::1]:1025/notifications/".parse()?)
    ///     .handshake_base_path("hand/") // http://[::1]:1025/notifications/hand/handshake
    ///     .build()?;
    /// # Ok(()) };
    /// ```
    #[inline(always)]
    pub fn handshake_base_path(self, url: &'b str) -> Self {
        Self {
            handshake_base_path: url,
            ..self
        }
    }

    /// Set cometd server subscribe url path.
    ///
    /// # Example
    /// ```rust
    /// use cometd_client::CometdClientBuilder;
    ///
    /// # let _ = || -> cometd_client::types::CometdResult<_> {
    /// let app = CometdClientBuilder::new(&"http://[::1]:1025/notifications/".parse()?)
    ///     .subscribe_base_path("sub/") // http://[::1]:1025/notifications/sub/
    ///     .build()?;
    /// # Ok(()) };
    /// ```
    #[inline(always)]
    pub fn subscribe_base_path(self, url: &'c str) -> Self {
        Self {
            subscribe_base_path: url,
            ..self
        }
    }

    /// Set cometd server connect url path.
    ///
    /// # Example
    /// ```rust
    /// use cometd_client::CometdClientBuilder;
    ///
    /// # let _ = || -> cometd_client::types::CometdResult<_> {
    /// let app = CometdClientBuilder::new(&"http://[::1]:1025/notifications/".parse()?)
    ///     .connect_base_path("con/") // http://[::1]:1025/notifications/con/connect
    ///     .build()?;
    /// # Ok(()) };
    /// ```
    #[inline(always)]
    pub fn connect_base_path(self, url: &'d str) -> Self {
        Self {
            connect_base_path: url,
            ..self
        }
    }

    /// Set cometd server disconnect url path.
    ///
    /// # Example
    /// ```rust
    /// use cometd_client::CometdClientBuilder;
    ///
    /// # let _ = || -> cometd_client::types::CometdResult<_> {
    /// let app = CometdClientBuilder::new(&"http://[::1]:1025/notifications/".parse()?)
    ///     .connect_base_path("discon/") // http://[::1]:1025/notifications/discon/disconnect
    ///     .build()?;
    /// # Ok(()) };
    /// ```
    #[inline(always)]
    pub fn disconnect_base_path(self, url: &'e str) -> Self {
        Self {
            disconnect_base_path: url,
            ..self
        }
    }

    /// Set `timeout` option in handshake request.
    #[inline(always)]
    pub fn timeout_ms(self, timeout_ms: u64) -> Self {
        Self {
            timeout_ms: Some(timeout_ms),
            ..self
        }
    }

    /// Set `interval` option in handshake request.
    #[inline(always)]
    pub fn interval_ms(self, interval_ms: u64) -> Self {
        Self {
            interval_ms: Some(interval_ms),
            ..self
        }
    }

    /// Set `interval` option in handshake request.
    #[inline(always)]
    pub fn access_token<AT>(self, access_token: AT) -> Self
    where
        AT: AccessToken + 'static,
    {
        Self {
            access_token: Some(Box::new(access_token)),
            ..self
        }
    }
}
