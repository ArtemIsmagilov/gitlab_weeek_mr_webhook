use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use log::info;

use crate::constants::RE_WEEEK_TASK_IDS;
use crate::structures::MergeRequest;
use crate::utils::{is_valid_token, push_mr};

#[post("/")]
pub async fn index(req: HttpRequest, mr: web::Json<MergeRequest>) -> impl Responder {
    let token = match req.headers().get("X-Gitlab-Token") {
        Some(t) => t,
        None => {
            info!("Missing header X-Gitlab-Token");
            return HttpResponse::PreconditionFailed();
        }
    };
    if !is_valid_token(token) {
        info!("X-Gitlab-Token header is not valid");
        return HttpResponse::PreconditionFailed();
    };
    if mr.event != "merge_request" || mr.action != "merge" {
        info!("Merge request attributes does not match event: '{}' and action: '{}'. Expected event: merge_request, action: merge", mr.event, mr.action);
        return HttpResponse::PreconditionFailed();
    }
    let weeek_ids: Vec<usize> = RE_WEEEK_TASK_IDS
        .captures_iter(&mr.title)
        .map(|c| {
            let (_, [weeek_id]) = c.extract();
            weeek_id.parse().unwrap()
        })
        .collect();
    if weeek_ids.is_empty() {
        info!(
            "Merge request title: '{}' does not match. Expected title by example: [3] Fix bug.",
            mr.title
        );
        return HttpResponse::PreconditionFailed();
    }
    push_mr(weeek_ids, mr.url.clone());
    HttpResponse::Accepted()
}
