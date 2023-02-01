use crate::types::{CometdError, Data};
use std::sync::Arc;

#[derive(Debug)]
pub enum CometdClientEvent<Msg> {
    Error(CometdError),
    Message(Arc<[Data<Msg>]>),
}

impl<Msg> CometdClientEvent<Msg> {
    #[inline(always)]
    pub(crate) fn message(data: Arc<[Data<Msg>]>) -> Arc<Self> {
        Arc::new(Self::Message(data))
    }

    #[inline(always)]
    pub(crate) fn error<E>(error: E) -> Arc<Self>
    where
        CometdError: From<E>,
    {
        Arc::new(Self::Error(CometdError::from(error)))
    }
}
