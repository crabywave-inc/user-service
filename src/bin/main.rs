use clap::Parser;

use std::sync::Arc;
use user::application::http::{HttpServer, HttpServerConfig};
use user::env::Env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let env = Arc::new(Env::parse());

    let server_config = HttpServerConfig::new(env.port.clone());
    let http_server = HttpServer::new(server_config).await?;

    http_server.run().await
}
