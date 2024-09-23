use fly_sdk::FlyControl;
use std::error::Error;
use tracing_subscriber::{filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stdout))
        .with(EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .init();
    let api_token = std::env::var("FLY_ORG_TOKEN").expect("FLY_ORG_TOKEN must be set");
    let args: Vec<String> = std::env::args().collect();
    let org_slug = &args
        .get(1)
        .expect("Usage: cargo run --example apps <org_slug>");
    
    let fly = FlyControl::new(api_token.to_string());
    
    let app_name = "rusty-app";
    fly.apps.create(app_name, org_slug).await?;
    fly.apps.list(org_slug).await?;
    fly.apps.delete(app_name, false).await?;

    Ok(())
}
