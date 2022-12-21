use crate::{
    consts::{DEFAULT_INTERVAL_MS, DEFAULT_TIMEOUT_MS},
    types::{AccessToken, CometdError, CometdResult},
    CometdClient,
};
use arc_swap::ArcSwapOption;
use hyper::Client;
use url::Url;

/// A builder to construct `CometdClient`.
#[derive(Debug, Default)]
pub struct CometdClientBuilder<'a, 'b, 'c, 'd, 'e> {
    endpoint: Option<&'a str>,
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
    pub fn new() -> Self {
        Self::default()
    }

    /// Return a `CometdClient`.
    ///
    /// # Example
    /// ```rust
    /// use cometd_client::CometdClientBuilder;
    ///
    /// # let _ = || -> cometd_client::types::CometdResult<_> {
    /// let client = CometdClientBuilder::new()
    ///     .endpoint("http://[::1]:1025/notifications/")
    ///     .build()?;
    /// # Ok(()) };
    /// ```
    #[inline(always)]
    pub fn build(self) -> CometdResult<CometdClient> {
        let Self {
            endpoint,
            handshake_base_path,
            subscribe_base_path,
            connect_base_path,
            disconnect_base_path,
            timeout_ms,
            interval_ms,
            access_token,
        } = self;

        let base_url = Url::parse(endpoint.ok_or(CometdError::MissingEndpoint)?)?;
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
        let cookie = Default::default();
        let client_id = Default::default();
        let http_client = Client::builder().build_http();

        Ok(CometdClient {
            handshake_endpoint,
            subscribe_endpoint,
            connect_endpoint,
            disconnect_endpoint,
            timeout_ms,
            interval_ms,
            id,
            access_token,
            cookie,
            client_id,
            http_client,
        })
    }

    /// Set cometd server endpoint.
    ///
    /// # Example
    /// ```rust
    /// use cometd_client::CometdClientBuilder;
    ///
    /// # let _ = || -> cometd_client::types::CometdResult<_> {
    /// let app = CometdClientBuilder::new()
    ///     .endpoint("http://[::1]:1025/notifications/")
    ///     .build()?;
    /// # Ok(()) };
    /// ```
    #[inline(always)]
    pub fn endpoint(self, url: &'a str) -> Self {
        Self {
            endpoint: Some(url),
            ..self
        }
    }

    /// Set cometd server handshake url path.
    ///
    /// # Example
    /// ```rust
    /// use cometd_client::CometdClientBuilder;
    ///
    /// # let _ = || -> cometd_client::types::CometdResult<_> {
    /// let app = CometdClientBuilder::new()
    ///     .handshake_base_path("hand/") // http://[::1]:1025/notifications/hand/handshake
    ///     .endpoint("http://[::1]:1025/notifications/")
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
    /// let app = CometdClientBuilder::new()
    ///     .subscribe_base_path("sub/") // http://[::1]:1025/notifications/sub/
    ///     .endpoint("http://[::1]:1025/notifications/")
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
    /// let app = CometdClientBuilder::new()
    ///     .connect_base_path("con/") // http://[::1]:1025/notifications/con/connect
    ///     .endpoint("http://[::1]:1025/notifications/")
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
    /// let app = CometdClientBuilder::new()
    ///     .connect_base_path("discon/") // http://[::1]:1025/notifications/discon/disconnect
    ///     .endpoint("http://[::1]:1025/notifications/")
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
