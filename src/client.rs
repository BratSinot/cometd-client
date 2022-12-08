mod builder;
mod connect;
mod disconnect;
mod handshake;
mod subscribe;

pub use builder::*;
use std::sync::Arc;

use crate::AccessToken;
use hyper::{
    client::HttpConnector, header::SET_COOKIE, http::HeaderValue, Body, Client, Response, Uri,
};
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct CometdClient {
    handshake_endpoint: Uri,
    subscribe_endpoint: Uri,
    connect_endpoint: Uri,
    disconnect_endpoint: Uri,
    timeout_ms: u64,
    interval_ms: u64,

    id: AtomicUsize,
    access_token: RwLock<Option<Box<dyn AccessToken>>>,
    cookie: RwLock<Option<HeaderValue>>,
    client_id: RwLock<Option<Arc<str>>>,
    http_client: Client<HttpConnector>,
}

impl CometdClient {
    #[inline]
    pub async fn update_access_token<AT>(&self, access_token: AT)
    where
        AT: AccessToken,
        Box<dyn AccessToken>: From<AT>,
    {
        *self.access_token.write().await = Some(access_token.into());
    }

    #[inline(always)]
    pub(crate) fn next_id(&self) -> String {
        self.id.fetch_add(1, Ordering::Relaxed).to_string()
    }

    #[inline(always)]
    pub(crate) async fn extract_and_store_cookie(&self, response: &mut Response<Body>) {
        if let Some(cookie) = response.headers_mut().remove(SET_COOKIE) {
            *self.cookie.write().await = Some(cookie);
        }
    }
}
