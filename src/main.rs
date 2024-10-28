use gitlab_weeek_mr_webhook::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run().await
}
