use issueflow::session::{build_claims, sign_token, verify_token};

#[test]
fn jwt_round_trips() {
    let secret = "test-secret-key-32-bytes!!!!!";
    let claims = build_claims(42, "user-123", "glpat-abc");

    let token = sign_token(&claims, secret).unwrap();
    let verified = verify_token(&token, secret).unwrap();

    assert_eq!(verified.user_id, 42);
    assert_eq!(verified.sub, "user-123");
    assert_eq!(verified.access_token, "glpat-abc");
}

#[test]
fn jwt_rejects_tampered_token() {
    let secret = "test-secret-key-32-bytes!!!!!";
    let claims = build_claims(1, "user-123", "glpat-abc");

    let token = sign_token(&claims, secret).unwrap();
    let tampered = format!("{}!", token);
    assert!(verify_token(&tampered, secret).is_err());
}

#[test]
fn jwt_rejects_wrong_secret() {
    let secret = "test-secret-key-32-bytes!!!!!";
    let wrong = "wrong-secret-key-32-bytes!!!!";
    let claims = build_claims(1, "user-123", "glpat-abc");

    let token = sign_token(&claims, secret).unwrap();
    assert!(verify_token(&token, wrong).is_err());
}
