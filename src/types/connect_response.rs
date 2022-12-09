/// Contains channel name of message received from cometd server.
#[derive(Debug)]
pub struct Data<Msg> {
    /// Channel name from which was received message.
    pub channel: Option<String>,
    /// Received message.
    pub message: Option<Msg>,
}
