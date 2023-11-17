use rmp_serde::Serializer;
use serde::{Deserialize, Serialize};
use std::io;
use tokio::net::TcpListener;

#[derive(Deserialize, Serialize, Debug)]
struct Position {
    t: String,
    x: i32,
    y: i32,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").await?;

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            match listen(&socket).await {
                Ok(_) => println!("Connection processed"),
                Err(e) => println!("Error processing connection: {}", e),
            }
        });
    }
}

async fn send_buf(buf: Vec<u8>, socket: &tokio::net::TcpStream) -> io::Result<()> {
    socket.writable().await?;

    match socket.try_write(buf.as_slice()) {
        Ok(n) => {
            println!("{} bytes written", n);
        }
        Err(e) => {
            println!("Error writing to socket: {}", e);
        }
    }

    Ok(())
}

async fn listen(socket: &tokio::net::TcpStream) -> io::Result<()> {
    loop {
        socket.readable().await?;

        let mut buf = [0; 1024];
        match socket.try_read(&mut buf) {
            Ok(n) => {
                println!("{} bytes read", n);
                println!("{:?}", &buf[..n]);

                let pos = Position {
                    t: "Pos".to_string(),
                    x: 1,
                    y: 2,
                };

                let mut buf = Vec::new();
                pos.serialize(&mut Serializer::new(&mut buf)).unwrap();
                buf.splice(0..0, buf.len().to_le_bytes());
                send_buf(buf, socket).await?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                println!("Error reading from socket: {}", e);
            }
        }
    }
}
