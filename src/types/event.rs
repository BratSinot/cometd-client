use crate::types::{CometdError, Data};
use std::sync::Arc;

/// Events getting from event receiver channel.
#[allow(missing_docs)]
#[derive(Debug)]
pub enum CometdClientEvent<Msg> {
    Message(Arc<[Data<Msg>]>),
    Error(Arc<CometdError>),
}

// rustc linter AGAIN give false positive on derive -_-"
impl<Msg> Clone for CometdClientEvent<Msg> {
    fn clone(&self) -> Self {
        match *self {
            Self::Message(ref data) => Self::Message(Arc::clone(data)),
            Self::Error(ref error) => Self::Error(Arc::clone(error)),
        }
    }
}

impl<Msg> CometdClientEvent<Msg> {
    #[inline(always)]
    pub(crate) fn error(error: CometdError) -> Self {
        Self::Error(Arc::new(error))
    }
}
