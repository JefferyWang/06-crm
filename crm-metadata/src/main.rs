use anyhow::Result;
use crm_metadata::{AppConfig, MetadataService};
use tonic::transport::Server;
use tracing::info;
use tracing_subscriber::{
    filter::LevelFilter, fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _,
};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let config = AppConfig::load().expect("Failed to load config");

    let addr = format!("[::1]:{}", config.server.port);
    let addr = addr.parse().unwrap();
    let svc = MetadataService::new(config).into_server();
    info!("MetadataService listening on {}", addr);

    Server::builder().add_service(svc).serve(addr).await?;
    Ok(())
}
