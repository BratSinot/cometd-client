use crate::{
    client::Inner,
    types::{CometdResult, Command, Event},
};
use async_broadcast::Sender as MpmcSender;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver as MpscReceiver;
use tokio::task::JoinHandle;

impl Inner {
    pub(crate) fn spawn_command_listener(
        self: Arc<Inner>,
        mut commands_rx: MpscReceiver<Command>,
        events_tx: MpmcSender<Arc<Event>>,
    ) {
        tokio::spawn(async move {
            let mut client_task_handler = None;

            while let Some(cmd) = commands_rx.recv().await {
                if let Err(error) = self
                    .handle_cmd(&mut client_task_handler, &events_tx, cmd)
                    .await
                {
                    let _ = events_tx.broadcast(Arc::new(Event::Error(error))).await;
                }
            }
        });
    }

    async fn handle_cmd(
        self: &Arc<Inner>,
        client_task_handler: &mut Option<JoinHandle<()>>,
        events_tx: &MpmcSender<Arc<Event>>,
        cmd: Command,
    ) -> CometdResult<()> {
        match cmd {
            Command::Handshake => {
                self._handshake().await?;

                if client_task_handler.is_none()
                    || client_task_handler.as_ref().map(JoinHandle::is_finished) == Some(true)
                {
                    *client_task_handler =
                        Some(Arc::clone(self).spawn_client_listener(events_tx.clone()));
                }

                Ok(())
            }
            Command::Subscribe(body) => self._subscribe(body).await,
            Command::Disconnect(body) => {
                self._disconnect(body).await?;
                let _ = events_tx.broadcast(Arc::new(Event::Disconnected)).await;
                Ok(())
            }
        }
    }

    fn spawn_client_listener(
        self: Arc<Inner>,
        events_tx: MpmcSender<Arc<Event>>,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                match self._connect().await {
                    Ok(data) => {
                        let _ = events_tx.broadcast(Arc::new(Event::Messages(data))).await;
                    }
                    Err(error) => {
                        let _ = events_tx.broadcast(Arc::new(Event::Error(error))).await;
                        break;
                    }
                }
            }
        })
    }
}
