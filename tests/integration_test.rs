use actix_web::http::header::ContentType;
use actix_web::{test, App};
use mockall::automock;
use mockall::predicate::eq;

use gitlab_weeek_mr_webhook::constants::X_GITLAB_TOKEN;
use gitlab_weeek_mr_webhook::services::index;

pub mod support;
use support::{currect_json, unlink_title_json, update_action_json, wrong_json};

#[actix_web::test]
async fn test_index_wrong_json() {
    let app = test::init_service(App::new().service(index)).await;
    let req = test::TestRequest::post()
        .uri("/")
        .insert_header(ContentType::json())
        .set_json(wrong_json())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn test_index_no_gitlab_token_header() {
    let app = test::init_service(App::new().service(index)).await;
    let req = test::TestRequest::post()
        .uri("/")
        .insert_header(ContentType::json())
        .set_json(currect_json())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 412);
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
    assert_eq!(resp.status(), 412);
}

#[actix_web::test]
async fn test_index_expect_miss_webhook() {
    let app = test::init_service(App::new().service(index)).await;
    let req = test::TestRequest::post()
        .uri("/")
        .insert_header(ContentType::json())
        .insert_header(("X-Gitlab-Token", (&*X_GITLAB_TOKEN).as_str()))
        .set_json(update_action_json())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 412);
}

#[actix_web::test]
async fn test_index_unlink_title() {
    let app = test::init_service(App::new().service(index)).await;
    let req = test::TestRequest::post()
        .uri("/")
        .insert_header(ContentType::json())
        .insert_header(("X-Gitlab-Token", (&*X_GITLAB_TOKEN).as_str()))
        .set_json(unlink_title_json())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 412);
}

#[actix_web::test]
async fn test_index_success() {
    #[automock]
    trait MyTrait {
        fn push_mr(&self, weeek_task_id: u64, url: String);
    }

    let mut mock = MockMyTrait::new();
    mock.expect_push_mr()
        .with(eq(3), eq("http://127.0.0.1:8000/1".to_string()))
        .return_const(());

    let app = test::init_service(App::new().service(index)).await;
    let req = test::TestRequest::post()
        .uri("/")
        .insert_header(ContentType::json())
        .insert_header(("X-Gitlab-Token", (&*X_GITLAB_TOKEN).as_str()))
        .set_json(currect_json())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 202);
}
