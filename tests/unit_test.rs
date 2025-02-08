use minijinja::render;
use serde_json::Value;

pub mod support;

use gitlab_weeek_mr_webhook::constants::{RE_WEEEK_TASK_IDS, WEEEK_PUSH_MR};
use support::currect_json_weeek_push_mr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_mr_template() {
        assert_eq!(
            serde_json::from_str::<Value>(&render!(
                WEEEK_PUSH_MR,
                url => "http://127.0.0.1:3000/merge_requests/3"
            ))
            .unwrap(),
            currect_json_weeek_push_mr()
        );
    }

    #[test]
    fn test_find_task_ids() {
        let result: Vec<_> = RE_WEEEK_TASK_IDS
            .find_iter("1[12] [12kl12] [9.] [3] ")
            .map(|m| m.as_str())
            .collect();
        assert_eq!(result, ["[12]", "[3]"]);
    }
}
