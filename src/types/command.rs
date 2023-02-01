#[derive(Debug)]
pub struct CometdClientCommand(pub(crate) CommandInner);

#[derive(Debug)]
pub(crate) enum CommandInner {
    Subscribe(Vec<String>),
}

impl CometdClientCommand {
    #[inline(always)]
    pub fn subscribe(subscriptions: Vec<String>) -> Self {
        Self(CommandInner::Subscribe(subscriptions))
    }
}
