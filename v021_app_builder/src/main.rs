use clap::Parser;
use v021_app_builder::configuration::Configuration;
use v021_app_builder::app_builder::run_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Configuration::parse();
    run_app(config).await
}