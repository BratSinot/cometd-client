#[derive(Debug)]
pub struct Data<Msg> {
    pub channel: Option<String>,
    pub message: Option<Msg>,
}
