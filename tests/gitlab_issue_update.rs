use issueflow::gitlab::issues::encode_update_issue_body;

#[test]
fn update_issue_request_encodes_description_field() {
    let body = encode_update_issue_body("new body");

    assert_eq!(body, serde_json::json!({ "description": "new body" }));
}
