use actix_web::http::header::HeaderValue;
use actix_web::rt::spawn;

use crate::constants::X_GITLAB_TOKEN;
use crate::tasks::add_mrs_to_weeek;

pub fn is_valid_token(token: &HeaderValue) -> bool {
    token == &*X_GITLAB_TOKEN
}

pub fn push_mr(weeek_task_ids: Vec<usize>, url: String) {
    spawn(add_mrs_to_weeek(weeek_task_ids, url));
}
