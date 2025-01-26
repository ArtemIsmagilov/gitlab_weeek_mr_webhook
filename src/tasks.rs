use crate::client::WeeekClient;
use futures::future::join_all;
use log::{info, warn};

pub async fn add_mrs_to_weeek(task_weeek_ids: Vec<usize>, url: String) {
    info!("Running task with task_weeek_id: {task_weeek_ids:?} and url: {url}.");
    let ac = WeeekClient::new();
    let response_login = match ac.weeek_login().await {
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
    let resps = task_weeek_ids
        .iter()
        .map(|id| ac.weeek_push_comment(*id, &url));
    for response_push_mr in join_all(resps).await {
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
