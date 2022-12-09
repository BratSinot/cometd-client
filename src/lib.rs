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
    clippy::if_let_mutex
)]

//! This crate aims to make a client for CometD protocol.
//!
//! This project is in progress and might change a lot from version to version.
//!

mod client;
mod common;
mod consts;
mod ext;

/// Contains various structs, enums and traits.
pub mod types;

pub use client::*;

pub(crate) use ext::*;
