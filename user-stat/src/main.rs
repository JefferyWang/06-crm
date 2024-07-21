use anyhow::Result;
use tonic::transport::Server;
use tracing::info;
use tracing_subscriber::{
    filter::LevelFilter, fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _,
};
use user_stat::{AppConfig, UserStatsService};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let config = AppConfig::load().expect("Failed to load config");

    let addr = format!("[::1]:{}", config.server.port);
    let addr = addr.parse().unwrap();
    let svc = UserStatsService::new(config).await.into_server();
    info!("UserStatServer listening on {}", addr);

    Server::builder().add_service(svc).serve(addr).await?;
    Ok(())
}
