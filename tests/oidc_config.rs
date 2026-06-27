use issueflow::{
    config::raw::RawOidcConfig,
    oidc::{OidcConfig, issue_state, validate_state},
};

#[tokio::test]
async fn oidc_disabled_skips_required_field_validation() {
    let config = OidcConfig::from_raw(RawOidcConfig {
        enabled: Some(false),
        ..RawOidcConfig::default()
    })
    .await
    .unwrap();

    assert!(!config.is_enabled());
}

#[test]
fn oidc_state_round_trips_for_the_same_issuer() {
    let issuer = "https://gitlab.example.com";
    let signing_secret = "state-secret";

    let state = issue_state(issuer, signing_secret).unwrap();

    assert!(validate_state(&state, issuer, signing_secret).is_ok());
    assert!(validate_state(&state, "https://github.example.com", signing_secret).is_err());
}
