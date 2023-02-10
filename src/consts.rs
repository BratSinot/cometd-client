use core::time::Duration;

pub(crate) const DEFAULT_TIMEOUT_MS: Duration = Duration::from_secs(60);
pub(crate) const DEFAULT_INTERVAL_MS: Duration = Duration::new(0, 0);
pub(crate) const DEFAULT_EVENT_CHANNEL_CAPACITY: usize = 500;
pub(crate) const DEFAULT_COMMAND_CHANNEL_CAPACITY: usize = 2;
pub(crate) const DEFAULT_NUMBER_OF_REHANDSHAKE: usize = 3;
pub(crate) const DEFAULT_CLIENT_TIMEOUT: Duration = Duration::from_secs(60 * 5);

pub(crate) const APPLICATION_JSON: &str = "application/json";
