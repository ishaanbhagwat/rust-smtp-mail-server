use std::io;

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Start the SMTP server
    let listener = TcpListener::bind("127.0.0.1:2525").await?;
    println!("SMTP server running on 127.0.0.1:2525");

    loop {
        // Accept a new client connection
        let (mut socket, addr) = listener.accept().await?;
        println!("New connection from: {}", addr);

        tokio::spawn(async move {
            if let Err(e) = handle_client(&mut socket).await {
                eprintln!("Error handling client {}: {}", addr, e);
            }
        });
    }
}

async fn handle_client(socket: &mut tokio::net::TcpStream) -> io::Result<()> {
    let mut buffer = [0; 4096]; // Increased buffer size for larger inputs
    let mut message = String::new();

    let mut smtp_mail_from = String::new();
    let mut smtp_rcpt_to = String::new();
    let mut smtp_data = String::new(); // To accumulate email body content

    // Send initial SMTP greeting
    socket.write_all(b"220 SimpleSMTP Server Ready\r\n").await?;
    socket.flush().await?;

    loop {
        let bytes_read = socket.read(&mut buffer).await?;
        if bytes_read == 0 {
            break; // Connection closed by the client
        }

        message.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));

        // Replace escaped sequences with actual line breaks
        let processed_message = message.replace("\\r\\n", "\r\n");

        // Split the processed message into commands
        let lines: Vec<&str> = processed_message.split("\r\n").collect();

        // Reset the message buffer for unprocessed data
        message.clear();

        for command in lines {
            if command.is_empty() {
                continue;
            }

            println!("Processing command: {}", command);

            match command.split_whitespace().next() {
                Some("HELO") => {
                    socket.write_all(b"250 Hello, pleased to meet you\r\n").await?;
                    continue
                }
                Some("MAIL") if command.starts_with("MAIL FROM:") => {
                    smtp_mail_from = command[12..command.len()].to_string();
                    socket.write_all(b"250 Sender OK\r\n").await?;
                    continue
                }
                Some("RCPT") if command.starts_with("RCPT TO:") => {
                    smtp_rcpt_to = command[10..command.len()].to_string();
                    socket.write_all(b"250 Recipient OK\r\n").await?;
                    continue
                }
                Some("DATA") => {
                    socket.write_all(b"354 Start mail input; end with <CRLF>.<CRLF>\r\n").await?;
                    smtp_data.clear(); // Clear previous data and prepare for body input
                    continue
                }
                Some(".") => {
                    // Handle end of email data (i.e., the user sent a single dot)
                    socket.write_all(b"250 Message accepted\r\n").await?;

                    // Here, smpt_data contains the full body of the email
                    println!("Received email body:\n{}", smtp_data);

                    // After handling, clear smpt_data for future messages
                    smtp_data.clear();
                    continue
                }
                Some("QUIT") => {
                    socket.write_all(b"221 Bye\r\n").await?;
                    print!("Mail from: {}", smtp_mail_from);
                    print!("Mail to: {}", smtp_rcpt_to);
                    print!("Email body:\n{}", smtp_data);
                    return Ok(());
                }
                _ => {
                    let response = "500 Command not recognized\r\n";
                    socket.write_all(response.as_bytes()).await?; 
                    continue;
                }
            }
        }
    }

    print!("Connection closed by client");
    Ok(())
}