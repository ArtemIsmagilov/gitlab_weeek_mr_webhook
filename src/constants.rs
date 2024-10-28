use std::sync::LazyLock;

use regex::Regex;

pub static RE_WEEEK_TASK_ID: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\[(?P<weeek_task_id>\d{1,20})\]").expect("Should be valid compiled regex.")
});

pub static X_GITLAB_TOKEN: LazyLock<String> =
    LazyLock::new(|| std::env::var("X_GITLAB_TOKEN").expect("Should be exists gitlab token env."));
pub static WEEEK_EMAIL: LazyLock<String> =
    LazyLock::new(|| std::env::var("WEEEK_EMAIL").expect("Should be exists weeek email env."));
pub static WEEEK_PASSWORD: LazyLock<String> = LazyLock::new(|| {
    std::env::var("WEEEK_PASSWORD").expect("Should be exists weeek password env.")
});

pub static HOST_PORT: LazyLock<String> =
    LazyLock::new(|| std::env::var("HOST_PORT").expect("Should be exists host port env."));
pub static WORKERS: LazyLock<usize> = LazyLock::new(|| {
    std::env::var("WORKERS")
        .expect("Should be exists workers env.")
        .parse::<usize>()
        .unwrap()
});
