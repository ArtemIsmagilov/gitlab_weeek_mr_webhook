use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use log::info;

use crate::constants::RE_WEEEK_TASK_ID;
use crate::structures::MergeRequest;
use crate::utils::{is_valid, push_mr};

#[post("/")]
pub async fn index(req: HttpRequest, mr: web::Json<MergeRequest>) -> impl Responder {
    match req.headers().get("X-Gitlab-Token") {
        Some(token) => match is_valid(token) {
            true => match mr.event == "merge_request" && mr.action == "open" {
                true => match RE_WEEEK_TASK_ID.captures(&mr.title) {
                    Some(caps) => {
                        push_mr(
                            caps["weeek_task_id"].parse::<u64>().unwrap(),
                            mr.url.clone(),
                        );
                        HttpResponse::Accepted()
                    }
                    None => {
                        info!("Merge request title: '{}' does not match. Expected title by example: [3] Fix bug.", mr.title);
                        HttpResponse::PreconditionFailed()
                    }
                },
                false => {
                    info!("Merge request attributes does not match event: '{}' and action: '{}'. Expexted event: merge_request, action: opened", mr.event, mr.action);
                    HttpResponse::PreconditionFailed()
                }
            },
            false => {
                info!("X-Gitlab-Token header is not valid");
                HttpResponse::PreconditionFailed()
            }
        },
        None => {
            info!("Missing header X-Gitlab-Token");
            HttpResponse::PreconditionFailed()
        }
    }
}
