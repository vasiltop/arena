#![warn(clippy::pedantic)]

use rmp_serde::Serializer;
use serde::{Deserialize, Serialize};
use std::io;
use tokio::net::TcpListener;

#[derive(Deserialize, Serialize, Debug)]
struct Position {
    #[serde(rename = "type")]
    t: String,
    x: i32,
    y: i32,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").await?;

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move { listen(&socket).await });
    }
}

async fn send_buf(buf: Vec<u8>, socket: &tokio::net::TcpStream) -> io::Result<()> {
    socket.writable().await?;

    socket.try_write(buf.as_slice())?;

    Ok(())
}

async fn listen(socket: &tokio::net::TcpStream) -> io::Result<()> {
    loop {
        socket.readable().await?;

        let mut buf = [0; 1024];
        match socket.try_read(&mut buf) {
            Ok(n) => {
                handle_packet(&buf, n);

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
                println!("Error reading from socket: {e}");
            }
        }
    }
}

fn handle_packet(packet: &[u8; 1024], size: usize) {
    let packet = &packet[..size];
    let mut position = 0;

    while position < packet.len() - 1 {
        let size = usize::from(packet[position]);
        handle_instruction(&packet[position + 1..=position + size]);
        position += size + 1;
    }
}

fn handle_instruction(packet: &[u8]) {
    println!("{:?}", rmp_serde::from_slice::<Position>(packet));
}
