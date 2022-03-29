use std::path::Path;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

pub async fn read_chat_history<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<(String, String)>, std::io::Error> {
    let chat_history_file = match OpenOptions::new().read(true).open(path).await {
        Ok(file) => file,
        Err(_) => return Ok(Vec::new()),
    };
    let reader = BufReader::new(chat_history_file);
    let mut lines = reader.lines();

    let mut chat_history = Vec::new();

    while let Some(line) = lines.next_line().await? {
        let line: Vec<_> = line.chars().collect();
        if let Some(separator_index) = line.iter().position(|c| *c == ':') {
            let nickname: String = line[..separator_index].iter().collect();
            let message: String = line[separator_index + 1..].iter().collect();

            chat_history.push((nickname, message));
        }
    }

    Ok(chat_history)
}

pub async fn write_chat_history<P: AsRef<Path>>(
    path: P,
    history: &[(String, String)],
) -> Result<(), std::io::Error> {
    let chat_history_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .await?;
    let mut writer = BufReader::new(chat_history_file);

    for (nickname, message) in history.iter() {
        writer
            .write_all(format!("{}:{}", nickname, message).as_bytes())
            .await?;
    }

    Ok(())
}
