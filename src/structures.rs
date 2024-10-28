use serde::Deserialize;

#[derive(Deserialize)]
pub struct MergeRequest {
    pub event: String,
    pub title: String,
    pub action: String,
    pub url: String,
}
