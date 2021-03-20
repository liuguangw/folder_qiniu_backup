use serde::Serialize;

#[derive(Serialize)]
pub struct PutPolicy {
    pub scope: String,
    pub deadline: u64,
    #[serde(rename(serialize = "returnBody"))]
    pub return_body: String,
}
