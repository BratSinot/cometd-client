use crate::types::CometdClientEvent;
use async_broadcast::{Receiver, RecvError};

/// Event channel receiver.
#[derive(Debug)]
pub struct CometdEventReceiver<Msg>(pub(crate) Receiver<CometdClientEvent<Msg>>);

impl<Msg> CometdEventReceiver<Msg> {
    /// Receive event from event channel.
    /// Return `None` if channel was closed.
    #[inline(always)]
    pub async fn recv(&mut self) -> Option<CometdClientEvent<Msg>> {
        match self.0.recv().await {
            Ok(data) => Some(data),
            Err(RecvError::Closed) => None,
            Err(RecvError::Overflowed(_)) => unreachable!(),
        }
    }
}
