#![warn(clippy::pedantic)]

mod packet;
mod socket;

use packet::{send_packet, Packet};
use socket::{PlayerData, Socket};
use std::{collections::HashMap, io, sync::Arc};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    let players = Arc::new(Mutex::new(HashMap::<u32, Arc<Socket>>::new()));

    let mut current_id = 0;

    loop {
        let (socket, _) = listener.accept().await?;

        let players = Arc::clone(&players);
        let p = Arc::clone(&players);

        tokio::spawn(async move {
            let s = socket::Socket {
                socket,
                players: p,
                id: current_id,
            };

            let mut p = players.lock().await;

            let socket = Arc::new(s);
            let s = Arc::clone(&socket);
            p.insert(current_id, s);

            send_packet(Packet::Init { id: current_id }, &socket.socket).await?;

            drop(p);
            socket.listen().await?;

            let mut p = players.lock().await;
            p.remove(&current_id);

            for (_, socket) in p.iter() {
                packet::send_packet(packet::Packet::Disconnect { id: socket.id }, &socket.socket)
                    .await?;
            }

            Ok::<_, io::Error>(())
        });

        current_id += 1;
    }
}
