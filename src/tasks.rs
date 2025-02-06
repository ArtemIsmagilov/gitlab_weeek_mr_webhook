use futures::future::join_all;
use log::{info, warn};
use reqwest::Client;
use std::time::Duration;

use crate::client::{weeek_login, weeek_push_comment};

pub async fn add_mrs_to_weeek(task_weeek_ids: Vec<usize>, url: String) {
    info!("Running task with task_weeek_id: {task_weeek_ids:?} and url: {url}.");
    let ac = Client::builder()
        .cookie_store(true)
        .timeout(Duration::new(5, 0))
        .build()
        .unwrap();
    let response_login = match weeek_login(&ac).await {
        Err(e) => {
            warn!("Failed to login. Error: {e}");
            return;
        }
        Ok(r) => r,
    };
    if response_login.status() != 200 {
        info!(
            "Response after login return status code != 200: {}",
            response_login.status()
        );
        return;
    };
    for response_push_mr in join_all(
        task_weeek_ids
            .iter()
            .map(|id| weeek_push_comment(&ac, id, &url)),
    )
    .await
    {
        match response_push_mr {
            Err(e) => warn!("Failed to create comment. Error: {}", e),
            Ok(r) => {
                if r.status() != 200 {
                    info!(
                        "Response after push merge request return status code != 200: {}",
                        r.status()
                    )
                }
            }
        }
    }
}
