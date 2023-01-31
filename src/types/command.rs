#[derive(Debug)]
pub(crate) enum Command {
    Handshake,
    Subscribe(String),
    Disconnect(String),
}
