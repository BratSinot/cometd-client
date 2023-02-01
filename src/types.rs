mod command;
mod error;
mod event;
mod message;

/// Contains different implementations for `AccessToken` trait.
pub mod access_token;

pub use {access_token::AccessToken, command::*, error::*, event::*, message::*};

pub type CometdEventReceiver<Msg> =
    async_broadcast::Receiver<std::sync::Arc<CometdClientEvent<Msg>>>;
pub type CometdCommandsSender = tokio::sync::mpsc::Sender<CometdClientCommand>;
