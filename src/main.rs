use issueflow::{config::Config, http::server};

#[tokio::main]
async fn main() {
    let config = Config::load()
        .await
        .expect("failed to load gateway configuration");
    server::serve(config)
        .await
        .expect("failed to run gateway server");
}
