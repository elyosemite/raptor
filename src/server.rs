mod proxy;

use crate::balancer::Balancer;
use tokio::net::TcpListener;

pub async fn run(
    listener: TcpListener,
    mut balancer: Balancer
) -> anyhow::Result<()> {
    loop {
        let (client, _) =
            listener.accept().await?;
        
        let backend =
            balancer.next();

        tracing::info!(%backend, "routing connection");

        tokio::spawn(async move {
            if let Err(e) =
                proxy::handle(
                    client,
                    backend
                ).await
            {
                tracing::error!(
                    "proxy error: {}",
                    e
                );
            }
        });
    }
}