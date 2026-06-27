use axum::Json;
use serde::Serialize;

use crate::{error::AppError, session::Session};

#[derive(Serialize)]
pub struct MeResponse {
    pub user_id: i64,
    pub sub: String,
}

pub async fn me(session: Session) -> Result<Json<MeResponse>, AppError> {
    Ok(Json(MeResponse {
        user_id: session.user_id,
        sub: session.sub,
    }))
}
