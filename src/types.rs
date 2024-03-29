mod channel;
mod command;
mod error;
mod event;
mod message;

/// Contains different implementations for `AccessToken` trait.
pub mod access_token;

use tokio::sync::mpsc;

pub(crate) use command::*;
pub use {access_token::AccessToken, channel::*, error::*, event::*, message::*};

pub(crate) type InactiveEventReceiver<Msg> =
    async_broadcast::InactiveReceiver<CometdClientEvent<Msg>>;
pub(crate) type EventSender<Msg> = async_broadcast::Sender<CometdClientEvent<Msg>>;

pub(crate) type CmdReceiver = mpsc::Receiver<Command>;
pub(crate) type CmdSender = mpsc::Sender<Command>;
