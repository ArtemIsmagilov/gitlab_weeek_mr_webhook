use serde_json::json;

pub fn currect_json() -> serde_json::Value {
    json!({
      "event": "merge_request",
      "title": "[3] Fix bug.",
      "url": "http://127.0.0.1:3000/merge_requests/3",
      "action": "merge"
    })
}

pub fn currect_json_weeek_push_mr() -> serde_json::Value {
    json!({
    "parentId": null,
    "content": {
        "type": "doc",
        "content": [
            {
                "type": "paragraph",
                "content": [
                    {
                        "type": "text",
                        "text": "http://127.0.0.1:3000/merge_requests/3"
                    }
                ]
            }
        ]
    }
    })
}

pub fn wrong_json() -> serde_json::Value {
    json!({ "wrong": "data" })
}

pub fn unlink_title_json() -> serde_json::Value {
    json!({
      "event": "merge_request",
      "title": "Fix bug.",
      "url": "http://127.0.0.1:3000/merge_requests/3",
      "action": "merge"
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
