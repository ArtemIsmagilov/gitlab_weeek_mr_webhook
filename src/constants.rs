use std::sync::LazyLock;

use regex::Regex;

pub static RE_WEEEK_TASK_IDS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[(\d+)\]").expect("Should be valid compiled regex."));
pub const X_GITLAB_TOKEN: &str = env!("X_GITLAB_TOKEN");

pub const WEEEK_URL: &str = env!("WEEEK_URL");
pub const WEEEK_EMAIL: &str = env!("WEEEK_EMAIL");
pub const WEEEK_PASSWORD: &str = env!("WEEEK_PASSWORD");

pub const APP_HOST: &str = env!("APP_HOST");
pub const APP_PORT: &str = env!("APP_PORT");
pub const APP_WORKERS: &str = env!("APP_WORKERS");
