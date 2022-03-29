use tokio::net::TcpListener;
use tokio::net::ToSocketAddrs;

mod chat_history;
mod connected_client;
pub mod error;

use connected_client::ConnectedClient;
use error::ServerError;

pub struct Server {
    listener: TcpListener,
    clients: Option<[ConnectedClient; 2]>,
    chat_name: String,
    chat_history: Vec<(String, String)>,
}

impl Server {
    pub async fn bind<A: ToSocketAddrs>(addr: A, chat_name: &str) -> Result<Server, ServerError> {
        if !crate::util::is_correct_chat_name(chat_name) {
            return Err(ServerError::Custom("Invalid chat name".to_string()));
        }

        Ok(Server {
            listener: TcpListener::bind(addr).await?,
            clients: None,
            chat_name: chat_name.to_string(),
            chat_history: chat_history::read_chat_history(Server::chat_history_path(chat_name))
                .await?,
        })
    }

    fn chat_history_path(chat_name: &str) -> String {
        format!("{}.txt", chat_name)
    }

    fn format_chat_history_entry_message(nickname: &str, message: &str) -> String {
        format!("{}: {}", nickname, message)
    }

    fn format_chat_history_message(
        history: &[(String, String)],
        available_nicknames: &[&str],
    ) -> String {
        let mut result = String::new();

        for (nickname, message) in history.iter() {
            if available_nicknames.contains(&nickname.as_str()) {
                result.push_str(&Server::format_chat_history_entry_message(
                    nickname, message,
                ));
                result.push('\n');
            }
        }

        result
    }

    async fn handle_client_message(
        client: &mut ConnectedClient,
        other_client: &mut ConnectedClient,
        message: &str,
        chat_history: &mut Vec<(String, String)>,
    ) -> Result<(), ServerError> {
        match client.nickname.as_ref() {
            Some(nickname) => {
                let result_message = format!("{}: {}", nickname, message);

                chat_history.push((nickname.to_string(), message.to_string()));
                client.write_line(&result_message).await?;
                other_client.write_line(&result_message).await?;
            }
            None => {
                if crate::util::is_correct_nickname(&message) {
                    client.nickname = Some(message.to_string());
                    client.write_line(&format!("Hi, {}!", message)).await?;
                } else {
                    client
                        .write_line("Bad nickname. Must be non-empty ASCII alphanumeric string")
                        .await?;
                }

                if let (Some(nickname), Some(nickname_other)) =
                    (client.nickname.as_ref(), other_client.nickname.as_ref())
                {
                    let result_message = Server::format_chat_history_message(
                        &chat_history,
                        &[&nickname, &nickname_other],
                    );

                    client.write_line(&result_message).await?;
                    other_client.write_line(&result_message).await?;
                }
            }
        }

        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), ServerError> {
        let mut client0 = ConnectedClient::new_accepted(&self.listener).await?;
        client0.write_line(
            "Hello! Type your nickname (ASCII alphanumeric non-empty string) while we are waiting to other client",
        ).await?;
        let mut client1 = ConnectedClient::new_accepted(&self.listener).await?;
        client1
            .write_line("Hello! Type your nickname (ASCII alphanumeric non-empty string)")
            .await?;

        self.clients = Some([client0, client1]);

        loop {
            if let Some(&mut [ref mut client0, ref mut client1]) = self.clients.as_mut() {
                tokio::select! {
                    Ok(Some(message)) = client0.read_line() => {
                        if let Err(_) = Server::handle_client_message(client0, client1, &message, &mut self.chat_history).await {
                            let _ = client1.write_line("Other client disconnected. Stopping...");
                            break
                        }
                    }
                    Ok(Some(message)) = client1.read_line() => {
                        if let Err(_) = Server::handle_client_message(client1, client0, &message, &mut self.chat_history).await {
                            let _= client0.write_line("Other client disconnected. Stopping...");
                            break
                        }
                    },
                    else => break,
                }
            }
        }

        chat_history::write_chat_history(
            Server::chat_history_path(&self.chat_name),
            &self.chat_history,
        )
        .await?;

        Ok(())
    }
}
