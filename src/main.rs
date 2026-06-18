mod balancer;
mod config;
mod server;

use balancer::Balancer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let config = config::load()?;
    let balancer = Balancer::new(config.backends)?;

    server::run(config.listen, balancer).await
}
