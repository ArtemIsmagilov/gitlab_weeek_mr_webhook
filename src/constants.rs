use std::env::var;
use std::sync::LazyLock;

use regex::Regex;

pub static RE_WEEEK_TASK_IDS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[(\d+)\]").expect("Should be valid compiled regex."));

pub static X_GITLAB_TOKEN: LazyLock<String> =
    LazyLock::new(|| var("X_GITLAB_TOKEN").expect("Should be exists gitlab token env."));

pub static WEEEK_URL: LazyLock<String> =
    LazyLock::new(|| var("WEEEK_URL").expect("Should be exists weeek url env."));
pub static WEEEK_EMAIL: LazyLock<String> =
    LazyLock::new(|| var("WEEEK_EMAIL").expect("Should be exists weeek email env."));
pub static WEEEK_PASSWORD: LazyLock<String> =
    LazyLock::new(|| var("WEEEK_PASSWORD").expect("Should be exists weeek password env."));

pub static APP_HOST: LazyLock<String> =
    LazyLock::new(|| var("APP_HOST").expect("Should be exists host env."));
pub static APP_PORT: LazyLock<u16> = LazyLock::new(|| {
    var("APP_PORT")
        .expect("Should be exists port env.")
        .parse()
        .unwrap()
});
pub static APP_WORKERS: LazyLock<usize> = LazyLock::new(|| {
    var("APP_WORKERS")
        .expect("Should be exists workers env.")
        .parse()
        .unwrap()
});
