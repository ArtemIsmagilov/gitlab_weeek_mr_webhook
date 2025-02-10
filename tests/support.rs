use serde_json::json;

pub fn currect_json() -> serde_json::Value {
    json!({
      "event": "merge_request",
      "title": "[3] Fix bug.",
      "url": "http://127.0.0.1:3000/merge_requests/3",
      "action": "merge"
    })
}

pub fn wrong_json() -> serde_json::Value {
    json!({ "wrong": "data" })
}

pub fn wrong_event_json() -> serde_json::Value {
    json!({
      "event": "wrong",
      "title": "[3] Fix bug.",
      "url": "http://127.0.0.1:3000/merge_requests/3",
      "action": "merge"
    })
}

pub fn wrong_title_json() -> serde_json::Value {
    json!({
      "event": "merge_request",
      "title": "wrong",
      "url": "http://127.0.0.1:3000/merge_requests/3",
      "action": "merge"
    })
}

pub fn wrong_url_json() -> serde_json::Value {
    json!({
      "event": "merge_request",
      "title": "[3] Fix bug.",
      "url": "wrong",
      "action": "merge"
    })
}

pub fn wrong_action_json() -> serde_json::Value {
    json!({
      "event": "merge_request",
      "title": "[3] Fix bug.",
      "url": "http://127.0.0.1:3000/merge_requests/3",
      "action": "wrong"
    })
}

pub fn extra_json() -> serde_json::Value {
    json!({
      "event": "merge_request",
      "title": "Fix bug.",
      "url": "http://127.0.0.1:3000/merge_requests/3",
      "action": "merge",
      "extra": "extra"
    })
}
