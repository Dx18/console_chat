use tokio::io::{AsyncBufReadExt, BufStream};
use tokio::net::TcpStream;

pub async fn get_next_line(
    stream: &mut BufStream<TcpStream>,
) -> Result<Option<String>, std::io::Error> {
    let mut buffer = String::new();
    let bytes_read = stream.read_line(&mut buffer).await?;
    if let Some('\n') = buffer.chars().last() {
        buffer.pop();
    }
    Ok(if bytes_read == 0 { None } else { Some(buffer) })
}

pub fn is_correct_nickname(nickname: &str) -> bool {
    !nickname.is_empty() && nickname.chars().all(|c| c.is_alphanumeric())
}

pub fn is_correct_chat_name(chat_name: &str) -> bool {
    !chat_name.is_empty() && chat_name.chars().all(|c| c.is_alphanumeric())
}
