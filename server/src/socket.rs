use std::{
    io,
    sync::{Arc, Mutex},
};
use tokio::net::TcpStream;

use crate::packet;

pub struct Socket {
    pub socket: TcpStream,
    pub queue: Arc<Mutex<Vec<Socket>>>,
}

impl Socket {
    pub async fn listen(&self) -> io::Result<()> {
        loop {
            self.socket.readable().await?;

            let mut buf = [0; 1024];
            match self.socket.try_read(&mut buf) {
                Ok(n) => {
                    self.handle_packet(&buf, n);

                    let pos = packet::Packet::Position { x: 1, y: 2 };

                    packet::send_packet(pos, &self.socket).await?;
                }

                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }

                Err(e) => {
                    println!("Error reading from socket: {e}");
                    break Ok(());
                }
            }
        }
    }

    fn handle_packet(&self, packet: &[u8; 1024], size: usize) {
        let packet = &packet[..size];
        let mut position = 0;

        while position < packet.len() - 1 {
            let size = usize::from(packet[position]);
            self.handle_instruction(&packet[position + 1..=position + size]);
            position += size + 1;
        }
    }

    fn handle_instruction(&self, packet: &[u8]) {
        println!("{:?}", rmp_serde::from_slice::<packet::Packet>(packet));
    }
}
