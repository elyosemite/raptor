use raptor::balancer::Balancer;
use raptor::server;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn fake_backend(id: u8) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap().to_string();

    tokio::spawn(async move {
        while let Ok((mut socket, _)) = listener.accept().await {
            tokio::spawn(async move {
                let _ = socket.write_all(&[id]).await;
            });
        }
    });

    addr
}

#[tokio::test]
async fn distributes_connections_round_robin() {
    let backend_a = fake_backend(b'A').await;
    let backend_b = fake_backend(b'B').await;
    let backend_c = fake_backend(b'C').await;

    let balancer = Balancer::new(vec![backend_a, backend_b, backend_c]).unwrap();

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let proxy_addr = listener.local_addr().unwrap();

    tokio::spawn(server::run(listener, balancer));

    for expected_id in [b'A', b'B', b'C', b'A'] {
        let mut client = TcpStream::connect(proxy_addr).await.unwrap();

        let mut buf = [0u8; 1];
        client.read_exact(&mut buf).await.unwrap();

        assert_eq!(buf[0], expected_id);
    }
}
