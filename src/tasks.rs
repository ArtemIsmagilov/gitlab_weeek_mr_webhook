use log::{error, info};

use crate::client;

pub async fn add_mr_to_weeek(task_weeek_id: u64, url: String) {
    info!("Running task with task_weeek_id: {task_weeek_id} and url: {url}.");

    let aclient = client::WeeekClient::new();

    let response_login = match aclient.weeek_login().await {
        Err(e) => {
            error!("Failed to login. Error: {e}");
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
    }

    let response_push_mr = match aclient.weeek_push_comment(task_weeek_id, url).await {
        Err(e) => {
            error!("Failed to create comment. Error: {}", e);
            return;
        }
        Ok(r) => r,
    };

    if response_push_mr.status() != 200 {
        info!(
            "Response after push merge request return status code != 200: {}",
            response_push_mr.status()
        );
        return;
    }

    info!("Merge request successfully commented to weeek.");
}
