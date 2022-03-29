use tokio::io::{AsyncWriteExt, BufStream};
use tokio::net::TcpStream;
use tokio::net::ToSocketAddrs;
use tokio::sync::mpsc;

pub mod error;

use error::ClientError;

pub async fn run_client<A: ToSocketAddrs>(
    server_addr: A,
    mut user_input: mpsc::UnboundedReceiver<String>,
    user_output: mpsc::UnboundedSender<String>,
) -> Result<(), ClientError> {
    let mut stream = BufStream::new(TcpStream::connect(server_addr).await?);

    loop {
        tokio::select! {
            Ok(Some(message)) = crate::util::get_next_line(&mut stream) => {
                if user_output.send(message).is_err() {
                    break;
                }
            },
            Some(message) = user_input.recv() => {
                stream.write_all(message.as_bytes()).await?;
                stream.flush().await?;
            }
            else => break
        }
    }

    Ok(())
}
