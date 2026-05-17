use crate::config::Config;

pub async fn serve(config: Config) -> Result<(), std::io::Error> {
    let listener = tokio::net::TcpListener::bind(&config.listen_addr).await?;
    axum::serve(listener, super::routes::router()).await
}
