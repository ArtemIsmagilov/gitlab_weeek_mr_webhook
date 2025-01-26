use crate::constants::{WEEEK_EMAIL, WEEEK_PASSWORD};
use reqwest::Client;
use serde_json::json;
use std::time::Duration;

pub struct WeeekClient(Client);

impl WeeekClient {
    pub fn new() -> Self {
        Self(
            Client::builder()
                .cookie_store(true)
                .timeout(Duration::new(5, 0))
                .build()
                .unwrap(),
        )
    }

    pub async fn weeek_login(&self) -> Result<reqwest::Response, reqwest::Error> {
        self.0
            .post("https://api.weeek.net/auth/login")
            .json(&json!({"email": &*WEEEK_EMAIL, "password": &*WEEEK_PASSWORD}))
            .send()
            .await
    }

    pub async fn weeek_push_comment(
        &self,
        task_weeek_id: usize,
        url: &str,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.0
            .post(format!(
                "https://api.weeek.net/ws/277820/tm/tasks/{task_weeek_id}/comments"
            ))
            .json(&json!({
            "parentId": null,
            "content": {
                "type": "doc",
                "content": [
                    {
                        "type": "paragraph",
                        "content": [
                            {
                                "type": "text",
                                "text": url
                            }
                        ]
                    }
                ]
            }
            }))
            .send()
            .await
    }
}
