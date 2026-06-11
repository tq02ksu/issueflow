use axum::{
    body::{to_bytes, Body},
    http::{header, Request, StatusCode},
};
use issueflow::{
    config::Config,
    oauth::{OAuthConfig, OAuthProviderConfig},
};
use tower::ServiceExt;

#[tokio::test]
async fn oauth_login_redirects_to_gitlab_authorize_url_with_signed_state() {
    let app = issueflow::http::routes::router(test_config());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/auth/gitlab/login")
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
    assert!(location.contains(
        "redirect_uri=http%3A%2F%2Flocalhost%3A3000%2Fauth%2Fgitlab%2Fcallback"
    ));
    assert!(location.contains("response_type=code"));
    assert!(location.contains("scope=read_user%20api"));
    assert!(location.contains("state="));
}

#[tokio::test]
async fn oauth_login_returns_not_found_when_provider_is_not_configured() {
    let app = issueflow::http::routes::router(Config::for_tests("expected-token"));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/auth/gitlab/login")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn oauth_callback_rejects_invalid_state() {
    let app = issueflow::http::routes::router(test_config());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/auth/gitlab/callback?code=test-code&state=invalid-state")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn oauth_callback_accepts_valid_state_without_reflecting_code() {
    let app = issueflow::http::routes::router(test_config());
    let login_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/auth/gitlab/login")
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
                .uri(format!("/auth/gitlab/callback?code=test-code&state={state}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert!(response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.contains("text/html")));

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let html = String::from_utf8(body.to_vec()).unwrap();

    assert!(html.contains("<title>Issueflow OAuth Callback</title>"));
    assert!(!html.contains("test-code"));
}

fn test_config() -> Config {
    Config::for_tests("expected-token").with_oauth(OAuthConfig::for_tests(vec![
        OAuthProviderConfig::gitlab_for_tests(),
    ]))
}

fn extract_query_param(location: &str, key: &str) -> String {
    location
        .split('?')
        .nth(1)
        .unwrap()
        .split('&')
        .find_map(|pair| pair.split_once('='))
        .filter(|(name, _)| *name == key)
        .map(|(_, value)| value.to_string())
        .unwrap()
}
