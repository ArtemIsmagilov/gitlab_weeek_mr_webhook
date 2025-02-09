use reqwest::Client;
use serde_json::json;

use crate::constants::{WEEEK_EMAIL, WEEEK_PASSWORD};

pub async fn weeek_login(ac: &Client) -> Result<reqwest::Response, reqwest::Error> {
    ac.post("https://api.weeek.net/auth/login")
        .json(&json!({"email": WEEEK_EMAIL, "password": WEEEK_PASSWORD}))
        .send()
        .await
}

pub async fn weeek_push_comment(
    ac: &Client,
    task_weeek_id: &usize,
    url: &str,
) -> Result<reqwest::Response, reqwest::Error> {
    ac.post(format!(
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
