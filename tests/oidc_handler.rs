use axum::{
    body::Body,
    http::{header, Request, StatusCode},
};
use issueflow::{
    config::Config,
    oidc::{OidcConfig, OidcEnabledConfig, OidcMetadata},
};
use tower::ServiceExt;

#[tokio::test]
async fn oidc_login_redirects_to_the_discovered_authorization_endpoint() {
    let app = issueflow::http::routes::router(test_config());
    let response = app
        .oneshot(Request::builder().uri("/auth/login").body(Body::empty()).unwrap())
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
    assert!(location.contains(
        "redirect_uri=http%3A%2F%2F127.0.0.1%3A8080%2Fauth%2Fcallback"
    ));
    assert!(location.contains("response_type=code"));
    assert!(location.contains("scope=openid%20profile%20email"));
    assert!(location.contains("state="));
}

#[tokio::test]
async fn oidc_login_returns_service_unavailable_when_oidc_is_disabled() {
    let app = issueflow::http::routes::router(Config::for_tests("expected-token"));
    let response = app
        .oneshot(Request::builder().uri("/auth/login").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
}

#[tokio::test]
async fn oidc_callback_rejects_invalid_state() {
    let app = issueflow::http::routes::router(test_config());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/auth/callback?code=test-code&state=invalid-state")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn oidc_callback_redirects_to_the_frontend_oidc_result_route() {
    let app = issueflow::http::routes::router(test_config());
    let login_response = app
        .clone()
        .oneshot(Request::builder().uri("/auth/login").body(Body::empty()).unwrap())
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
                .uri(format!("/auth/callback?code=test-code&state={state}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(
        response
            .headers()
            .get(header::LOCATION)
            .and_then(|value| value.to_str().ok()),
        Some("/auth/callback/oidc?result=success")
    );
}

fn test_config() -> Config {
    Config::for_tests("expected-token").with_oidc(OidcConfig::Enabled(OidcEnabledConfig {
        issuer: "https://gitlab.example.com".to_string(),
        client_id: "gitlab-test-client".to_string(),
        client_secret: "gitlab-test-secret".to_string(),
        redirect_uri: "http://127.0.0.1:8080/auth/callback".to_string(),
        scopes: vec!["openid".to_string(), "profile".to_string(), "email".to_string()],
        state_signing_secret: "test-oidc-state-secret".to_string(),
        metadata: OidcMetadata {
            issuer: "https://gitlab.example.com".to_string(),
            authorization_endpoint: "https://gitlab.example.com/oauth/authorize".to_string(),
            token_endpoint: "https://gitlab.example.com/oauth/token".to_string(),
        },
    }))
}

fn extract_query_param(location: &str, key: &str) -> String {
    let query = location.split('?').nth(1).unwrap();
    let prefix = format!("{key}=");
    let value_start = query.find(&prefix).unwrap() + prefix.len();
    let value = &query[value_start..];

    value.split('&').next().unwrap().to_string()
}
