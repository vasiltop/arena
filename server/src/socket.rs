use crate::packet::{self, send_packet, Packet};
use rmp_serde::decode::Error;
use std::{collections::HashMap, io, sync::Arc};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct PlayerData {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Socket {
    pub id: u32,
    pub socket: TcpStream,
    pub players: Arc<Mutex<HashMap<u32, Arc<Socket>>>>,
}

impl Socket {
    pub async fn send_to_all_except_self(
        &self,
        packet: packet::Packet,
        p: &HashMap<u32, Arc<Socket>>,
    ) -> io::Result<()> {
        for (id, socket) in p.iter() {
            if *id == self.id {
                continue;
            }

            packet::send_packet(packet, &socket.socket).await?;
        }

        Ok(())
    }

    pub async fn send_to_all(
        &self,
        packet: packet::Packet,
        p: &HashMap<u32, Arc<Socket>>,
    ) -> io::Result<()> {
        for (_, socket) in p.iter() {
            packet::send_packet(packet, &socket.socket).await?;
        }

        Ok(())
    }

    pub async fn listen(&self) -> io::Result<()> {
        loop {
            self.socket.readable().await?;

            let mut buf = [0; 1024];
            match self.socket.try_read(&mut buf) {
                Ok(n) => {
                    self.handle_packet(&buf, n).await;
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

    async fn handle_packet(&self, packet: &[u8; 1024], size: usize) {
        let packet = &packet[..size];
        let mut position = 0;

        while position < packet.len() - 1 {
            let size = usize::from(packet[position]);

            if position + size + 1 > packet.len() {
                break;
            }

            self.handle_instruction(&packet[position + 1..=position + size])
                .await
                .ok();
            position += size + 1;
        }
    }

    async fn handle_instruction(&self, packet: &[u8]) -> Result<(), Error> {
        println!(
            "Packet received: {:?}",
            rmp_serde::from_slice::<packet::Packet>(packet)
        );

        Ok(match rmp_serde::from_slice::<packet::Packet>(packet)? {
            Packet::Pos { id, x, y } => {
                let p = self.players.lock().await;
                let socket = p.get(&id).unwrap();

                socket
                    .send_to_all_except_self(Packet::Pos { id, x, y }, &p)
                    .await
                    .ok();
            }

            Packet::Sprite { id, sprite } => {
                let p = self.players.lock().await;
                let socket = p.get(&id).unwrap();

                socket
                    .send_to_all_except_self(Packet::Sprite { id, sprite }, &p)
                    .await
                    .ok();
            }
            Packet::Dir { id, dir, x_scale } => {
                let p = self.players.lock().await;
                let socket = p.get(&id).unwrap();

                socket
                    .send_to_all_except_self(Packet::Dir { id, dir, x_scale }, &p)
                    .await
                    .ok();
            }
            Packet::Shot { id } => {
                let p = self.players.lock().await;
                let socket = p.get(&id).unwrap();

                socket
                    .send_to_all_except_self(Packet::Shot { id }, &p)
                    .await
                    .ok();
            }
            Packet::Dmg { id, amount } => {
                let p = self.players.lock().await;
                let socket = p.get(&id).unwrap();

                socket
                    .send_to_all(Packet::Dmg { id, amount }, &p)
                    .await
                    .ok();
            }
            Packet::Death { id } => {
                let p = self.players.lock().await;
                let socket = p.get(&id).unwrap();

                socket.send_to_all(Packet::Death { id }, &p).await.ok();
            }
            _ => {}
        })
    }
}
