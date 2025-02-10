use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{test, App};

use gitlab_weeek_mr_webhook::constants::X_GITLAB_TOKEN;
use gitlab_weeek_mr_webhook::services::{healthcheck, index};

pub mod support;
use support::{
    currect_json, extra_json, wrong_action_json, wrong_event_json, wrong_json, wrong_title_json,
    wrong_url_json,
};

// test index service
#[actix_web::test]
async fn test_index_no_gitlab_token_header() {
    let app = test::init_service(App::new().service(index)).await;
    let req = test::TestRequest::post()
        .uri("/")
        .insert_header(ContentType::json())
        .set_json(currect_json())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::PRECONDITION_FAILED);
}

#[actix_web::test]
async fn test_index_no_correct_gitlab_token_header() {
    let app = test::init_service(App::new().service(index)).await;
    let req = test::TestRequest::post()
        .uri("/")
        .insert_header(ContentType::json())
        .insert_header(("X-Gitlab-Token", "wrong token"))
        .set_json(currect_json())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::PRECONDITION_FAILED);
}

#[actix_web::test]
async fn test_index_wrong_json() {
    let app = test::init_service(App::new().service(index)).await;
    let req = test::TestRequest::post()
        .uri("/")
        .insert_header(ContentType::json())
        .set_json(wrong_json())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn test_index_wrong_event() {
    let app = test::init_service(App::new().service(index)).await;
    let req = test::TestRequest::post()
        .uri("/")
        .insert_header(ContentType::json())
        .insert_header(("X-Gitlab-Token", X_GITLAB_TOKEN))
        .set_json(wrong_event_json())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::PRECONDITION_FAILED);
}

#[actix_web::test]
async fn test_index_wrong_title() {
    let app = test::init_service(App::new().service(index)).await;
    let req = test::TestRequest::post()
        .uri("/")
        .insert_header(ContentType::json())
        .insert_header(("X-Gitlab-Token", X_GITLAB_TOKEN))
        .set_json(wrong_title_json())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::PRECONDITION_FAILED);
}

#[actix_web::test]
async fn test_index_wrong_url() {
    let app = test::init_service(App::new().service(index)).await;
    let req = test::TestRequest::post()
        .uri("/")
        .insert_header(ContentType::json())
        .insert_header(("X-Gitlab-Token", X_GITLAB_TOKEN))
        .set_json(wrong_url_json())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::PRECONDITION_FAILED);
}

#[actix_web::test]
async fn test_index_wrong_action() {
    let app = test::init_service(App::new().service(index)).await;
    let req = test::TestRequest::post()
        .uri("/")
        .insert_header(ContentType::json())
        .insert_header(("X-Gitlab-Token", X_GITLAB_TOKEN))
        .set_json(wrong_action_json())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::PRECONDITION_FAILED);
}

#[actix_web::test]
async fn test_index_extra_fields() {
    let app = test::init_service(App::new().service(index)).await;
    let req = test::TestRequest::post()
        .uri("/")
        .insert_header(ContentType::json())
        .insert_header(("X-Gitlab-Token", X_GITLAB_TOKEN))
        .set_json(extra_json())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::PRECONDITION_FAILED);
}

#[actix_web::test]
async fn test_index_success() {
    let app = test::init_service(App::new().service(index)).await;
    let req = test::TestRequest::post()
        .uri("/")
        .insert_header(ContentType::json())
        .insert_header(("X-Gitlab-Token", X_GITLAB_TOKEN))
        .set_json(currect_json())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::ACCEPTED);
}
// test healthcheck service
#[actix_web::test]
async fn test_healtcheck_no_gitlab_token_header() {
    let app = test::init_service(App::new().service(healthcheck)).await;
    let req = test::TestRequest::get().uri("/healthcheck").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::PRECONDITION_FAILED);
}

#[actix_web::test]
async fn test_healtcheck_no_correct_gitlab_token_header() {
    let app = test::init_service(App::new().service(healthcheck)).await;
    let req = test::TestRequest::get()
        .uri("/healthcheck")
        .insert_header(("X-Gitlab-Token", "wrong token"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::PRECONDITION_FAILED);
}

#[actix_web::test]
async fn test_healtcheck_success() {
    let app = test::init_service(App::new().service(healthcheck)).await;
    let req = test::TestRequest::get()
        .uri("/healthcheck")
        .insert_header(("X-Gitlab-Token", X_GITLAB_TOKEN))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}
