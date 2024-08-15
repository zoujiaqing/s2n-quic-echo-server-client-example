use s2n_quic::Server;
use std::path::Path;

pub static CERT_PEM: &str = "certs/cert.pem";
pub static KEY_PEM: &str = "certs/key.pem";

use anyhow::Result;

pub async fn run() -> Result<()> {
    let mut server = Server::builder()
        .with_tls((Path::new(CERT_PEM), Path::new(KEY_PEM)))?
        .with_io("127.0.0.1:4433")?
        .start()?;

    while let Some(mut connection) = server.accept().await {
        // spawn a new task for the connection
        tokio::spawn(async move {
            while let Ok(Some(mut stream)) = connection.accept_bidirectional_stream().await {
                // spawn a new task for the stream
                tokio::spawn(async move {
                    // echo any data back to the stream
                    while let Ok(Some(data)) = stream.receive().await {
                        stream.send(data).await.expect("stream should be open");
                    }
                });
            }
        });
    }

    Ok(())
}
