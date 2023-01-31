use crate::types::{CometdError, Data};

#[allow(missing_docs)]
#[derive(Debug)]
pub enum Event {
    Messages(Vec<Data>),
    Error(CometdError),
    Disconnected,
}
