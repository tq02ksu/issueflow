mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode, header},
};
use issueflow::{
    config::Config,
    oidc::{OidcConfig, OidcEnabledConfig, OidcMetadata},
};
use tower::ServiceExt;

#[tokio::test]
async fn oidc_login_redirects_to_the_discovered_authorization_endpoint() {
    let app = common::test_app(test_config()).await;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/auth/login")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);

    let location = response
        .headers()
        .get(header::LOCATION)
        .and_then(|value| value.to_str().ok())
        .unwrap();

    assert!(location.starts_with("https://gitlab.example.com/oauth/authorize?"));
    assert!(location.contains("client_id=gitlab-test-client"));
    assert!(location.contains("redirect_uri=http%3A%2F%2F127.0.0.1%3A8080%2Fauth%2Fcallback"));
    assert!(location.contains("response_type=code"));
    assert!(location.contains("scope=openid%20profile%20email%20api%20read_repository%20ai_features"));
    assert!(location.contains("state="));
}

#[tokio::test]
async fn oidc_login_returns_service_unavailable_when_oidc_is_disabled() {
    let app = common::test_app(Config::for_tests("expected-token")).await;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/auth/login")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
}

#[tokio::test]
async fn oidc_callback_rejects_invalid_state() {
    let app = common::test_app(test_config()).await;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/auth/callback?code=test-code&state=invalid-state")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn oidc_callback_redirects_to_the_frontend_oidc_result_route() {
    let app = common::test_app(test_config()).await;
    let login_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/auth/login")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let state = extract_query_param(
        login_response
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok())
            .unwrap(),
        "state",
    );

    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/api/auth/callback?code=test-code&state={state}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
    let location = response
        .headers()
        .get(header::LOCATION)
        .and_then(|value| value.to_str().ok())
        .unwrap();
    assert!(location.starts_with("/auth/callback/oidc?result="));
}

fn test_config() -> Config {
    Config::for_tests("expected-token").with_oidc(OidcConfig::Enabled(OidcEnabledConfig {
        issuer: "https://gitlab.example.com".to_string(),
        client_id: "gitlab-test-client".to_string(),
        client_secret: "gitlab-test-secret".to_string(),
        redirect_uri: "http://127.0.0.1:8080/auth/callback".to_string(),
        scopes: vec![
            "openid".to_string(),
            "profile".to_string(),
            "email".to_string(),
            "api".to_string(),
            "read_repository".to_string(),
            "ai_features".to_string(),
        ],
        state_signing_secret: "test-oidc-state-secret".to_string(),
        metadata: Some(OidcMetadata {
            issuer: "https://gitlab.example.com".to_string(),
            authorization_endpoint: "https://gitlab.example.com/oauth/authorize".to_string(),
            token_endpoint: "https://gitlab.example.com/oauth/token".to_string(),
        }),
    }))
}

fn extract_query_param(location: &str, key: &str) -> String {
    let query = location.split('?').nth(1).unwrap();
    let prefix = format!("{key}=");
    let value_start = query.find(&prefix).unwrap() + prefix.len();
    let value = &query[value_start..];

    value.split('&').next().unwrap().to_string()
}
