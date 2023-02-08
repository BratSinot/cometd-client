#![deny(unused_must_use)]
#![warn(
    rust_2018_idioms,
    rust_2021_compatibility,
    missing_docs,
    missing_debug_implementations,
    clippy::expect_used,
    clippy::missing_panics_doc,
    clippy::panic_in_result_fn,
    clippy::panicking_unwrap,
    clippy::unwrap_used,
    clippy::if_let_mutex,
    clippy::std_instead_of_core,
    clippy::missing_const_for_fn,
    clippy::str_to_string,
    clippy::clone_on_ref_ptr,
    clippy::panic,
    clippy::explicit_iter_loop,
    clippy::pattern_type_mismatch,
    clippy::indexing_slicing,
    clippy::use_debug,
    clippy::unnested_or_patterns,
    clippy::return_self_not_must_use,
    clippy::map_unwrap_or,
    clippy::items_after_statements,
    clippy::needless_pass_by_value,
    clippy::if_not_else,
    clippy::option_if_let_else
)]

//! This crate aims to make a client for CometD protocol.
//!
//! This project is in progress and might change a lot from version to version.
//!
//! # Table of contents
//! - [Connect endpoints](#connect-endpoints)
//! - [Authentication](#authentication)
//! - [Authentication through authorization header](#authorization-authentication)
//! - [Authentication through cookie](#cookie-authentication)
//! - [How to interact with client?](#interaction-with-client)
//!
//! # Connect endpoints
//!
//! Client has ability to customize endpoints base paths:
//! 1) [`CometdClientBuilder::handshake_base_path`];
//! 2) [`CometdClientBuilder::subscribe_base_path`];
//! 3) [`CometdClientBuilder::connect_base_path`];
//! 4) [`CometdClientBuilder::disconnect_base_path`];
//!
//! For example to change handshake base path and
//! get `http://[::1]:1025/notifications/node/0/handshake`
//! you can do this:
//! ```rust,no_run
//! use cometd_client::CometdClientBuilder;
//!
//! # || -> Result<(), Box<dyn std::error::Error>> {
//! let client = CometdClientBuilder::new(&"http://[::1]:1025/notifications/".parse()?)
//!     .handshake_base_path("hand/")
//!     .build()?;
//! # Ok(())
//! # };
//! ```
//!
//! Same for others endpoints.
//!
//! # Authentication
//!
//! There is 2 options to authenticate on server,
//! through `authorization` header and cookie.
//!
//! # Authentication through authorization header
//!
//! To use access token with `authorization` header,
//! you must set it through [`CometdClientBuilder::access_token`].
//! This library provide 2 default structs:
//! [`types::access_token::Bearer`] and [`types::access_token::Basic`].
//! ```rust,no_run
//! use cometd_client::{CometdClientBuilder, types::access_token::{Bearer, Basic}};
//!
//! # || -> Result<(), Box<dyn std::error::Error>> {
//! // let access_token = Bearer::new("access-token");
//! let access_token = Basic::create("username", Some("optional password"))?;
//!
//! let client = CometdClientBuilder::new(&"http://[::1]:1025/notifications/".parse()?)
//!     .access_token(access_token)
//!     .build()?;
//! # Ok(())
//! # };
//! ```
//!
//! But you can make you own access token for `authorization` header:
//! ```rust,no_run
//! use core::fmt::{Formatter, Debug};
//! use cometd_client::types::AccessToken;
//!
//! struct OAuth(String);
//!
//! impl Debug for OAuth {
//!     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//!         write!(f, "OAuth(***)")
//!     }
//! }
//!
//! impl OAuth {
//!     pub fn new(access_token: &str) -> Self {
//!         Self(format!("OAuth {access_token}"))
//!     }
//! }
//!
//! impl AccessToken for OAuth {
//!     fn get_authorization_token(&self) -> &str {
//!         &self.0
//!     }
//! }
//! ```
//!
//! # Authentication through cookie
//!
//! If you use session cookies for authentication (or other reasons) you can set it (or them)
//! through [`CometdClientBuilder::cookie`] or [`CometdClientBuilder::cookies`].
//! ```rust,no_run
//! use cometd_client::CometdClientBuilder;
//!
//! # || -> Result<(), Box<dyn std::error::Error>> {//!
//! let client = CometdClientBuilder::new(&"http://[::1]:1025/notifications/".parse()?)
//!     .cookie("cookie0-name", "cookie0-value")
//!     .cookies([
//!         ("cookie1-name", "cookie1-value"),
//!         ("cookie2-name", "cookie2-value")
//!     ])
//!     .build()?;
//! # Ok(())
//! # };
//! ```
//!
//! # How to interact with client?
//!
//! Client use MPMC channel to send messages and errors.
//! [`CometdClientBuilder::build`] spawn task which do handshake and start wait for messages.
//! If handshake request was unsuccessful with [`types::Reconnect::Handshake`] or [`types::Reconnect::Retry`] advice from server,
//! then client tries redo it by [`CometdClientBuilder::number_of_retries`] times.
//! In other cases task send error to event channel and stops.
//!
//! After successful handshake task start listen messages coming from server.
//! If during that requests occurs error with [`types::Reconnect::Handshake`] advice,
//! then client will tries redo handshake (look above).
//! If error will be with [`types::Reconnect::Retry`] advice, then it will try redo it
//! by [`CometdClientBuilder::number_of_retries`] times.
//!
//! To send subscribe command you must use [`CometdClient::subscribe`].
//! If error occurs it will be redone by same scheme as for connect (look above).
//!
//! To get event channel receiver use [`CometdClient::rx`].
//!
//! ```rust
//! use cometd_client::{types::CometdClientEvent, CometdClientBuilder};
//!
//! # async fn _test() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CometdClientBuilder::new(&"http://[::0]:1025/notifications/".parse()?)
//!     .build()?;
//!
//! let mut rx = client.rx();
//!
//! tokio::spawn(async move {
//!     while let Ok(event) = rx.recv().await {
//!         match event {
//!             CometdClientEvent::Message(messages) => println!("got messages: `{messages:?}`."),
//!             CometdClientEvent::Error(error) => eprintln!("got error: `{error:?}`."),   
//!         }   
//!     }
//! });
//!
//! # Ok(())
//! # }
//! ```
//!

mod client;
mod common;
mod consts;
mod ext;
mod sugar;

/// Contains various structs, enums and traits.
pub mod types;

pub use client::*;

pub(crate) use {ext::*, sugar::*};
