use issueflow::session::{build_claims, sign_token};

pub fn auth_token(jwt_secret: &str, user_id: i64, sub: &str, access_token: &str) -> String {
    let claims = build_claims(user_id, sub, access_token);
    sign_token(&claims, jwt_secret).unwrap()
}
