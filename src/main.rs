use issueflow::{config::Config, http::server};

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    server::serve(config)
        .await
        .expect("failed to run gateway server");
}
