use actix_web::rt::spawn;
use actix_web::HttpRequest;
use log::info;

use crate::constants::X_GITLAB_TOKEN;
use crate::tasks::add_mrs_to_weeek;

pub fn push_mr(weeek_task_ids: Vec<usize>, url: String) {
    spawn(add_mrs_to_weeek(weeek_task_ids, url));
}

pub fn auth_token(req: &HttpRequest) -> bool {
    let token = match req.headers().get("X-Gitlab-Token") {
        Some(t) => t,
        None => {
            info!("Missing header X-Gitlab-Token");
            return false;
        }
    };
    if token != X_GITLAB_TOKEN {
        info!("X-Gitlab-Token header is not valid");
        return false;
    };
    true
}
