use serde_json::Value as JsonValue;

#[derive(Debug)]
pub(crate) enum Command {
    Subscribe(JsonValue),
}
