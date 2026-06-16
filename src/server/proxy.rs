use tokio::net::TcpStream;
use tokio::io::copy_bidirectional;

pub async fn handle(
    mut client: TcpStream,
    backend: String
)
-> anyhow::Result<()> {
    let mut server =
        TcpStream::connect(
            backend
        ).await?;
    
    copy_bidirectional(
        &mut client,
        &mut server
    ).await?;

    Ok(())
}