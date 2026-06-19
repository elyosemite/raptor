mod balancer;
mod config;
mod server;

use balancer::Balancer;

const DEFAULT_CONFIG_PATH: &str = "raptor.toml";

fn resolve_config_path(args: &[String]) -> String {
    args.get(1)
        .cloned()
        .unwrap_or_else(|| DEFAULT_CONFIG_PATH.to_string())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    let path = resolve_config_path(&args);

    let config = config::load(&path)?;
    let balancer = Balancer::new(config.backends)?;

    server::run(config.listen, balancer).await
}

#[cfg(test)]
mod tests;
