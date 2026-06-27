use crate::{config::Config, db::DbPool, http::routes::AppState};

pub async fn serve(config: Config, pool: DbPool) -> Result<(), std::io::Error> {
    let state = AppState { config, pool };
    let listener = tokio::net::TcpListener::bind(&state.config.listen_addr).await?;
    axum::serve(listener, super::routes::router(state)).await
}
