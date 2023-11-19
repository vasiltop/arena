use crate::packet;
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
    pub player_data: Arc<Mutex<HashMap<u32, PlayerData>>>,
}

impl Socket {
    pub async fn listen(&self) -> io::Result<()> {
        self.socket.writable().await?;
        let pd = self.player_data.lock().await;
        //Set the current sockets position
        packet::send_packet(
            packet::Packet::Initialize {
                id: self.id,
                x: pd.get(&self.id).unwrap().x,
                y: pd.get(&self.id).unwrap().y,
            },
            &self.socket,
        )
        .await?;

        let p = self.players.lock().await;

        //Set other sockets positions
        for (id, socket) in p.iter() {
            println!("{id:?} {:?}", self.id);
            if *id == self.id {
                continue;
            }

            packet::send_packet(
                packet::Packet::Position {
                    id: socket.id,
                    x: pd.get(&socket.id).unwrap().x,
                    y: pd.get(&socket.id).unwrap().y,
                },
                &self.socket,
            )
            .await?;
        }

        //Send the current sockets position to other sockets
        for (id, socket) in p.iter() {
            if *id == self.id {
                continue;
            }

            packet::send_packet(
                packet::Packet::Position {
                    id: self.id,
                    x: pd.get(&self.id).unwrap().x,
                    y: pd.get(&self.id).unwrap().y,
                },
                &socket.socket,
            )
            .await?;
        }

        drop(p);
        drop(pd);
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
            println!("Size: {size}");
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
            packet::Packet::Input {
                up,
                down,
                left,
                right,
            } => {
                let mut pd = self.player_data.lock().await;
                let data = pd.get_mut(&self.id).unwrap();

                if up {
                    data.y -= 5;
                }

                if down {
                    data.y += 5;
                }

                if left {
                    data.x -= 5;
                }

                if right {
                    data.x += 5;
                }

                let p = self.players.lock().await;

                for (id, socket) in p.iter() {
                    if *id == self.id {
                        continue;
                    }

                    packet::send_packet(
                        packet::Packet::Position {
                            id: self.id,
                            x: data.x,
                            y: data.y,
                        },
                        &socket.socket,
                    )
                    .await
                    .ok();
                }
            }

            _ => {}
        })
    }
}
