use console_chat;

use std::io::BufRead;
use tokio::sync::mpsc;

mod error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Should create server [h] or connect to existing one [c]?");

    let create_server = loop {
        let stdin = std::io::stdin();
        let mut input = String::new();
        if stdin.lock().read_line(&mut input)? == 0 {
            return Ok(());
        }

        let input = input.trim();

        if input == "h" {
            break true;
        } else if input == "c" {
            break false;
        }

        println!("Type 'h' or 'c'");
    };

    println!("Type address of the server:");
    let address = loop {
        let stdin = std::io::stdin();
        let mut input = String::new();
        if stdin.lock().read_line(&mut input).unwrap() == 0 {
            return Ok(());
        }

        let input = input.trim();

        if !input.is_empty() {
            break input.to_string();
        }
    };

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()?;

    let (stdin_send, stdin_recv) = mpsc::unbounded_channel();
    let (stdout_send, mut stdout_recv) = mpsc::unbounded_channel();

    let stdin_handle = std::thread::spawn(move || {
        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            let line = match line {
                Err(_) => break,
                Ok(line) => line,
            };
            if let Err(_) = stdin_send.send(format!("{}\n", line)) {
                break;
            }
        }
    });

    let stdout_handle = std::thread::spawn(move || {
        while let Some(line) = stdout_recv.blocking_recv() {
            println!("{}", line);
        }
    });

    let main_handle = runtime.spawn(async move {
        let mut server = if create_server {
            Some(console_chat::server::Server::bind(address.clone(), "default").await?)
        } else {
            None
        };

        let client_handle = tokio::spawn(async move {
            console_chat::client::run_client(address, stdin_recv, stdout_send).await
        });

        if let Some(server) = server.as_mut() {
            server.run().await?;
        }

        if let Ok(result) = client_handle.await {
            result?;
        }

        Ok::<_, error::Error>(())
    });

    if let Err(err) = runtime.block_on(main_handle)? {
        println!("An error occurred: {}", err);
        println!("Type something to exit");
    }

    stdin_handle.join().unwrap();
    stdout_handle.join().unwrap();

    Ok(())
}
