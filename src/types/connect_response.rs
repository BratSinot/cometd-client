use crate::types::Reconnect;

#[derive(Debug)]
pub struct ConnectResponse<Msg> {
    pub reconnect: Reconnect,
    pub data: Vec<Data<Msg>>,
}

#[derive(Debug)]
pub struct Data<Msg> {
    pub channel: Option<String>,
    pub message: Option<Msg>,
}
