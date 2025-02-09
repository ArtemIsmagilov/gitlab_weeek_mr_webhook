use gitlab_weeek_mr_webhook::constants::RE_WEEEK_TASK_IDS;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_task_ids() {
        let result: Vec<_> = RE_WEEEK_TASK_IDS
            .find_iter("1[12] [12kl12] [9.] [3] ")
            .map(|m| m.as_str())
            .collect();
        assert_eq!(result, ["[12]", "[3]"]);
    }
}
