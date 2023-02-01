use crate::{consts::*, types::CometdResult, CometdClient, CometdClientBuilder, CookieJarExt};
use arc_swap::ArcSwapOption;
use hyper::Client;
use std::sync::Arc;
use tokio::sync::RwLock;

impl<'a, 'b, 'c, 'd, 'e> CometdClientBuilder<'a, 'b, 'c, 'd, 'e> {
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
            cookies,
            ..
        } = self;

        let handshake_endpoint =
            String::from(base_url.join(handshake_base_path)?.join("handshake")?)
                .try_into()
                .map_err(Arc::new)?;
        let subscribe_endpoint = String::from(base_url.join(subscribe_base_path)?)
            .try_into()
            .map_err(Arc::new)?;
        let connect_endpoint = String::from(base_url.join(connect_base_path)?.join("connect")?)
            .try_into()
            .map_err(Arc::new)?;
        let disconnect_endpoint =
            String::from(base_url.join(disconnect_base_path)?.join("disconnect")?)
                .try_into()
                .map_err(Arc::new)?;
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

        Ok(CometdClient {
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
        })
    }
}
