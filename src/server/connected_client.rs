use tokio::io::{AsyncWriteExt, BufStream};
use tokio::net::{TcpListener, TcpStream};

pub struct ConnectedClient {
    pub stream: BufStream<TcpStream>,
    pub nickname: Option<String>,
}

impl ConnectedClient {
    pub async fn new_accepted(listener: &TcpListener) -> Result<ConnectedClient, std::io::Error> {
        let (stream, _) = listener.accept().await?;
        Ok(ConnectedClient {
            stream: BufStream::new(stream),
            nickname: None,
        })
    }

    pub async fn read_line(&mut self) -> Result<Option<String>, std::io::Error> {
        crate::util::get_next_line(&mut self.stream).await
    }

    pub async fn write_line(&mut self, buffer: &str) -> Result<(), std::io::Error> {
        self.stream.write_all(buffer.as_bytes()).await?;
        self.stream.write_all(b"\n").await?;
        self.stream.flush().await
    }
}
