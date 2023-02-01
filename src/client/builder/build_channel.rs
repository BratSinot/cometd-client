use crate::{types::*, CometdClient, CometdClientBuilder};
use async_broadcast::{broadcast, Sender};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use tokio::{select, sync::mpsc};

impl<'a, 'b, 'c, 'd, 'e> CometdClientBuilder<'a, 'b, 'c, 'd, 'e> {
    /// Build client and spawn task which listen messages and send it to event channel.
    pub fn build_channel<Msg>(
        self,
    ) -> CometdResult<(CometdEventReceiver<Msg>, CometdCommandsSender)>
    where
        Msg: DeserializeOwned + Send + Sync + 'static,
    {
        let Self {
            commands_channel_capacity,
            events_channel_capacity,
            ..
        } = self;
        let client = self.build()?;

        let (event_tx, event_rx) = broadcast(events_channel_capacity);
        let (command_tx, mut command_rx) = mpsc::channel(commands_channel_capacity);

        tokio::spawn(async move {
            if let Err(error) = client.handshake().await {
                let _ = event_tx.broadcast(CometdClientEvent::error(error)).await;
            } else {
                loop {
                    select! {
                        biased;
                        cmd = command_rx.recv() => if handle_cmd(&client, &event_tx, cmd).await { break },
                        data = client.connect::<Msg>() => if handle_data(&event_tx, data).await { break },
                    }
                }

                if let Err(error) = client.disconnect().await {
                    let _ = event_tx.broadcast(CometdClientEvent::error(error)).await;
                }
            }
        });

        Ok((event_rx, command_tx))
    }
}

async fn handle_cmd<Msg>(
    client: &CometdClient,
    event_tx: &Sender<Arc<CometdClientEvent<Msg>>>,
    cmd: Option<CometdClientCommand>,
) -> bool {
    if let Some(cmd) = cmd {
        let res = match cmd {
            CometdClientCommand(CommandInner::Subscribe(subscriptions)) => {
                client.subscribe(&subscriptions).await
            }
        };

        if let Err(error) = res {
            let _ = event_tx.broadcast(CometdClientEvent::error(error)).await;
            true
        } else {
            false
        }
    } else {
        true
    }
}

async fn handle_data<Msg>(
    event_tx: &Sender<Arc<CometdClientEvent<Msg>>>,
    data: CometdResult<Arc<[Data<Msg>]>>,
) -> bool {
    match data {
        Ok(data) => {
            let _ = event_tx.broadcast(CometdClientEvent::message(data)).await;
            false
        }
        Err(error) => {
            let _ = event_tx.broadcast(CometdClientEvent::error(error)).await;
            true
        }
    }
}
