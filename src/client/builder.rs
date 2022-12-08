use crate::{
    consts::{DEFAULT_INTERVAL_MS, DEFAULT_TIMEOUT_MS},
    CometdClient, CometdError, CometdResult,
};
use hyper::Client;
use url::Url;

#[derive(Debug, Default)]
pub struct CometdClientBuilder {
    base_url: Option<&'static str>,
    handshake_base_path: &'static str,
    subscribe_base_path: &'static str,
    connect_base_path: &'static str,
    disconnect_base_path: &'static str,
    timeout_ms: Option<u64>,
    interval_ms: Option<u64>,
}

impl CometdClientBuilder {
    /// Construct a new `ClientBuilder`.
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline(always)]
    pub fn build(self) -> CometdResult<CometdClient> {
        let Self {
            base_url,
            handshake_base_path,
            subscribe_base_path,
            connect_base_path,
            disconnect_base_path,
            timeout_ms,
            interval_ms,
        } = self;

        let base_url = Url::parse(base_url.ok_or(CometdError::MissingBaseUrl)?)?;
        let handshake_endpoint =
            String::from(base_url.join(handshake_base_path)?.join("handshake")?).try_into()?;
        let subscribe_endpoint = String::from(base_url.join(subscribe_base_path)?).try_into()?;
        let connect_endpoint =
            String::from(base_url.join(connect_base_path)?.join("connect")?).try_into()?;
        let disconnect_endpoint =
            String::from(base_url.join(disconnect_base_path)?.join("disconnect")?).try_into()?;

        Ok(CometdClient {
            handshake_endpoint,
            subscribe_endpoint,
            connect_endpoint,
            disconnect_endpoint,
            timeout_ms: timeout_ms.unwrap_or(DEFAULT_TIMEOUT_MS),
            interval_ms: interval_ms.unwrap_or(DEFAULT_INTERVAL_MS),
            id: Default::default(),
            access_token: Default::default(),
            cookie: Default::default(),
            client_id: Default::default(),
            http_client: Client::builder().build_http(),
        })
    }

    #[inline(always)]
    pub fn base_url(self, url: &'static str) -> Self {
        Self {
            base_url: Some(url),
            ..self
        }
    }

    #[inline(always)]
    pub fn subscribe_base_path(self, url: &'static str) -> Self {
        Self {
            subscribe_base_path: url,
            ..self
        }
    }

    #[inline(always)]
    pub fn handshake_base_path(self, url: &'static str) -> Self {
        Self {
            handshake_base_path: url,
            ..self
        }
    }

    #[inline(always)]
    pub fn connect_base_path(self, url: &'static str) -> Self {
        Self {
            connect_base_path: url,
            ..self
        }
    }

    #[inline(always)]
    pub fn disconnect_base_path(self, url: &'static str) -> Self {
        Self {
            disconnect_base_path: url,
            ..self
        }
    }

    #[inline(always)]
    pub fn timeout_ms(self, timeout_ms: u64) -> Self {
        Self {
            timeout_ms: Some(timeout_ms),
            ..self
        }
    }

    #[inline(always)]
    pub fn interval_ms(self, interval_ms: u64) -> Self {
        Self {
            interval_ms: Some(interval_ms),
            ..self
        }
    }
}
