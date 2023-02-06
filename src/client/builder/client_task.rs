use crate::{client::CometdClientInner, retry_with_advice, types::*};
use core::future::{ready, Future};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use tokio::select;

#[inline(always)]
pub(crate) fn spawn(
    inner: CometdClientInner,
    cmd_rx: CmdReceiver,
    event_tx: EventSender<impl DeserializeOwned + Send + Sync + 'static>,
) {
    tokio::task::spawn(async move {
        let broadcast_event = |event| async {
            let _ = event_tx.broadcast(event).await;
        };

        if let Err(error) = retry_with_advice(
            inner.number_of_retries,
            || ready(Ok(())),
            || inner.handshake(),
        )
        .await
        {
            broadcast_event(CometdClientEvent::error(error)).await;
        } else {
            cmd_connect_loop(inner, cmd_rx, broadcast_event).await;
        }
    });
}

#[inline(always)]
async fn cmd_connect_loop<Msg, Fut>(
    inner: CometdClientInner,
    mut cmd_rx: CmdReceiver,
    broadcast_event: impl Fn(CometdClientEvent<Msg>) -> Fut,
) where
    Msg: DeserializeOwned,
    Fut: Future<Output = ()>,
{
    enum Res<Msg> {
        Left(Option<Command>),
        Right(CometdResult<Arc<[Data<Msg>]>>),
    }
    use Res::*;

    loop {
        let res = select! {
            biased;
            cmd = cmd_rx.recv() => Left(cmd),
            data = retry_with_advice(
                inner.number_of_retries,
                || inner.handshake(),
                || inner.connect::<Msg>(),
            ) => Right(data),
        };

        match res {
            Left(Some(Command::Subscribe(subscriptions))) => {
                if let Err(error) = retry_with_advice(
                    inner.number_of_retries,
                    || inner.handshake(),
                    || inner.subscribe(&subscriptions),
                )
                .await
                {
                    broadcast_event(CometdClientEvent::error(error)).await;
                    break;
                }
            }
            Right(Ok(data)) => broadcast_event(CometdClientEvent::Message(data)).await,
            // communication errors
            Left(None) => break,
            Right(Err(error)) => {
                broadcast_event(CometdClientEvent::error(error)).await;
                break;
            }
        }
    }

    if let Err(error) = inner.disconnect().await {
        broadcast_event(CometdClientEvent::error(error)).await;
    }
}
