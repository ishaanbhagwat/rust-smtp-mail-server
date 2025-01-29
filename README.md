# SMTP Server in Rust
An SMTP server in Rust, designed for learning and experimentation. The server listens for incoming client connections, processes SMTP commands, and handles basic email communication. It was built using the Tokio async runtime for handling concurrent connections.

## Features
- Accepts `HELO`, `MAIL FROM`, `RCPT TO`, `DATA`, and `QUIT` commands.

- Processes email content and outputs it to the console.

- Handles multiple client connections concurrently using tokio::spawn.

- Gracefully closes client connections.

## Requirements
- Rust (1.70 or higher recommended)
- Tokio crate for async networking

## Key Concepts and Learning Highlights

### 1. Rust's Ownership and Borrowing Model

The Rust ownership model is built for memory safety without a garbage collector. 
During the implementation:
Borrowing (&mut) was used extensively when handling the TCP socket.

Ownership of variables like smtp_mail_from and smtp_rcpt_to was managed to avoid invalid references.

### 2. Async Programming with Tokio

The tokio crate was used for:

Asynchronous TCP networking (`tokio::net::TcpListener` and `tokio::net::TcpStream`).

Handling concurrent client connections with `tokio::spawn`.

Non-blocking I/O with `AsyncReadExt` and `AsyncWriteExt`.

### 3. SMTP Protocol Basics

Implemented core SMTP commands (`HELO`, `MAIL FROM`, `RCPT TO`, `DATA`, `QUIT`).

Learned how SMTP clients and servers exchange messages and manage email transactions.

## How to Run
#### 1. Clone Repository
```bash
git clone https://github.com/your-username/smtp-server-rust.git
cd smtp-server-rust
```

#### 2. Build and Run Server
```bash
cargo run
```

#### 3. Connect to Server using Telnet
```bash
telnet 127.0.0.1 2525
```

#### 4. Use SMTP commands to interact with the server
```bash
HELO localhost
MAIL FROM:<example@domain.com>
RCPT TO:<recipient@domain.com>
DATA
This is a test email.
.
QUIT
```

## Planned Further Improvements
- Add authentication support (e.g., LOGIN/PLAIN mechanisms).

- Persist emails to a database or filesystem.

- Support additional SMTP commands like VRFY and EXPN.

- Implement a publisher/subscriber model for email retrieval.
