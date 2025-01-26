use serde_json::json;

pub fn wrong_json() -> serde_json::Value {
    json!({ "wrong": "data" })
}

pub fn currect_json() -> serde_json::Value {
    json!({
      "event": "merge_request",
      "title": "[3] Fix bug.",
      "url": "http://127.0.0.1:3000/merge_requests/3",
      "action": "merge"
    })
}

pub fn unlink_title_json() -> serde_json::Value {
    json!({
      "event": "merge_request",
      "title": "Fix bug.",
      "url": "http://127.0.0.1:3000/merge_requests/3",
      "action": "merge"
    })
}
