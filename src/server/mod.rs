mod proxy;

use crate::balancer::Balancer;
use tokio::net::TcpListener;

pub async fn run(
    address: String,
    mut balancer: Balancer
) -> anyhow::Result<()> {
    let listener =
        TcpListener::bind(address).await?;
    
    loop {
        let (client, _) =
            listener.accept().await?;
        
        let backend =
            balancer.next();
        
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