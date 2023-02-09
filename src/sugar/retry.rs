use crate::types::{CometdError, CometdResult, ErrorKind, Reconnect};
use core::future::Future;

pub(crate) async fn retry_with_advice<T, Fut, HandshakeFut>(
    number_of_retries: usize,
    handshake: impl Fn() -> HandshakeFut,
    f: impl Fn() -> Fut,
) -> CometdResult<T>
where
    Fut: Future<Output = CometdResult<T>>,
    HandshakeFut: Future<Output = CometdResult<()>>,
{
    let mut f_retries = number_of_retries;

    loop {
        match f().await {
            Ok(ret) => break Ok(ret),
            Err(CometdError::WrongResponse(kind, Reconnect::Handshake, _)) => {
                check_retries(kind, f_retries)?;
                handshake_retry(number_of_retries, &handshake).await?;
            }
            Err(CometdError::WrongResponse(kind, Reconnect::Retry, _)) => {
                check_retries(kind, f_retries)?
            }
            Err(error) => break Err(error),
        }
        f_retries -= 1;
    }
}

#[inline(always)]
async fn handshake_retry<HandshakeFut>(
    mut handshake_retries: usize,
    handshake: impl Fn() -> HandshakeFut,
) -> CometdResult<()>
where
    HandshakeFut: Future<Output = CometdResult<()>>,
{
    loop {
        match handshake().await {
            Ok(()) => break Ok(()),
            Err(CometdError::WrongResponse(kind, Reconnect::Handshake | Reconnect::Retry, _)) => {
                check_retries(kind, handshake_retries)?
            }
            Err(error) => break Err(error),
        }
        handshake_retries -= 1;
    }
}

#[inline(always)]
fn check_retries(kind: ErrorKind, retries: usize) -> CometdResult<()> {
    retries
        .gt(&0)
        .then_some(())
        .ok_or(CometdError::wrong_response(
            kind,
            Reconnect::None,
            "exhausted attempts",
        ))
}
