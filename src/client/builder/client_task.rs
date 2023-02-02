use crate::{client::CometdClientInner, types::*};
use core::future::Future;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use tokio::select;

#[inline(always)]
pub(crate) fn spawn<Msg>(inner: CometdClientInner, cmd_rx: CmdReceiver, event_tx: EventSender<Msg>)
where
    Msg: DeserializeOwned + Send + Sync + 'static,
{
    tokio::task::spawn(async move {
        let broadcast_event = |event| async {
            let _ = event_tx.broadcast(event).await;
        };

        if let Err(error) = inner.handshake().await {
            broadcast_event(CometdClientEvent::error(error)).await;
        } else {
            cmd_connect_loop(inner, cmd_rx, broadcast_event).await;
        }
    });
}

#[inline(always)]
async fn cmd_connect_loop<Msg, F, Fut>(
    inner: CometdClientInner,
    mut cmd_rx: CmdReceiver,
    broadcast_event: F,
) where
    Msg: DeserializeOwned,
    F: Fn(CometdClientEvent<Msg>) -> Fut,
    Fut: Future<Output = ()>,
{
    enum Res<Msg> {
        Left(Option<Command>),
        Right(CometdResult<Arc<[Data<Msg>]>>),
    }
    use Res::*;

    let error = loop {
        let res = select! {
            biased;
            cmd = cmd_rx.recv() => Left(cmd),
            data = inner.connect::<Msg>() => Right(data),
        };

        match res {
            Left(Some(Command::Subscribe(subscriptions))) => {
                if let Err(error) = inner.subscribe(subscriptions).await {
                    break error;
                }
            }
            Right(Ok(data)) => broadcast_event(CometdClientEvent::message(data)).await,
            // communication errors
            Left(None) => break CometdError::Internal("internal command channel was closed"),
            Right(Err(error)) => break error,
        }
    };

    broadcast_event(CometdClientEvent::error(error)).await;
    if let Err(error) = inner.disconnect().await {
        broadcast_event(CometdClientEvent::error(error)).await;
    }
}
