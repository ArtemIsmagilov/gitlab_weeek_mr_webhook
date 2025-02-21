use actix_web::{HttpRequest, HttpResponse, Responder, get, post, web};
use log::info;
use url::Url;

use crate::constants::RE_WEEEK_TASK_IDS;
use crate::structures::MergeRequest;
use crate::utils::{auth_token, push_mr};

#[post("/")]
pub async fn index(req: HttpRequest, mr_input: web::Json<MergeRequest>) -> impl Responder {
    if !auth_token(&req) {
        return HttpResponse::PreconditionFailed();
    };
    let mr = mr_input.into_inner();
    if mr.event != "merge_request" {
        info!(
            "Merge request attribute does not match. event: {}. Expected event: merge_request.",
            mr.event
        );
        return HttpResponse::PreconditionFailed();
    }
    if mr.action != "merge" {
        info!(
            "Merge request attribute does not match. action: {}. Expected event: merge.",
            mr.action
        );
        return HttpResponse::PreconditionFailed();
    }
    if Url::parse(&mr.url).is_err() {
        info!(
            "Merge request attribute does not match. url: {}. Expected valid url.",
            mr.url
        );
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
    push_mr(weeek_ids, mr.url);
    HttpResponse::Accepted()
}

#[get("/healthcheck")]
pub async fn healthcheck(req: HttpRequest) -> impl Responder {
    if !auth_token(&req) {
        HttpResponse::PreconditionFailed()
    } else {
        HttpResponse::Ok()
    }
}
