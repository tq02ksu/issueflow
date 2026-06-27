use issueflow::session::{SessionClaims, sign_session, verify_session};

#[test]
fn session_round_trips() {
    let secret = b"test-secret-key-32-bytes!!!!!";
    let claims = SessionClaims {
        user_id: 42,
        sub: "user-123".to_string(),
        access_token: "glpat-abc".to_string(),
    };

    let token = sign_session(&claims, secret);
    let verified = verify_session(&token, secret).unwrap();

    assert_eq!(verified.user_id, 42);
    assert_eq!(verified.sub, "user-123");
    assert_eq!(verified.access_token, "glpat-abc");
}

#[test]
fn session_rejects_tampered_token() {
    let secret = b"test-secret-key-32-bytes!!!!!";
    let claims = SessionClaims {
        user_id: 1,
        sub: "user-123".to_string(),
        access_token: "glpat-abc".to_string(),
    };

    let token = sign_session(&claims, secret);
    let tampered = token.replace("a", "b");
    assert!(verify_session(&tampered, secret).is_err());
}

#[test]
fn session_rejects_wrong_secret() {
    let secret = b"test-secret-key-32-bytes!!!!!";
    let wrong = b"wrong-secret-key-32-bytes!!!!";
    let claims = SessionClaims {
        user_id: 1,
        sub: "user-123".to_string(),
        access_token: "glpat-abc".to_string(),
    };

    let token = sign_session(&claims, secret);
    assert!(verify_session(&token, wrong).is_err());
}
