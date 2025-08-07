use std::{io, net::SocketAddr};

use tokio::{io::AsyncWriteExt, net::TcpListener};
use tracing::{info, warn};

const BUF_SIZE: usize = 4096;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "0.0.0.0:6379";
    let listener = TcpListener::bind(addr).await?;
    info!("Dredis: listening on: {addr}");
    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("Accepted connection from: {}", raddr);
        tokio::spawn(async move {
            if let Err(e) = process_redis_conn(stream, raddr).await {
                warn!("Error processing conn with {raddr}: {e:?}");
            }
        });
    }
    #[allow(unreachable_code)]
    Ok(())
}

async fn process_redis_conn(
    mut stream: tokio::net::TcpStream,
    _raddr: SocketAddr,
) -> anyhow::Result<()> {
    loop {
        stream.readable().await?;
        let mut buf = Vec::with_capacity(BUF_SIZE);
        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                info!("read {n} bytes");
                let line = String::from_utf8_lossy(&buf);
                info!("{line:?}");
                stream.write_all(b"+OK\r\n").await?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => return Err(e.into()),
        }
    }

    #[allow(unreachable_code)]
    Ok(())
}
