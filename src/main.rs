use issueflow::{config::Config, db, http::server};

#[tokio::main]
async fn main() {
    let config = Config::load()
        .await
        .expect("failed to load gateway configuration");

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:issueflow.db?mode=rwc".to_string());
    let pool = db::open(&database_url)
        .await
        .expect("failed to open database");

    server::serve(config, pool)
        .await
        .expect("failed to run gateway server");
}
