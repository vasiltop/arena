#![warn(clippy::pedantic)]

mod packet;
mod socket;

use socket::{PlayerData, Socket};
use std::{collections::HashMap, io, sync::Arc};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    let players = Arc::new(Mutex::new(HashMap::<u32, Arc<Socket>>::new()));
    let player_data = Arc::new(Mutex::new(HashMap::<u32, PlayerData>::new()));
    let mut current_id = 0;

    loop {
        let (socket, _) = listener.accept().await?;

        let players = Arc::clone(&players);
        let player_data = Arc::clone(&player_data);

        tokio::spawn(async move {
            let s = socket::Socket {
                socket,
                players: Arc::clone(&players),
                id: current_id,
                player_data: Arc::clone(&player_data),
            };

            let mut p = players.lock().await;
            let s = Arc::new(s);
            p.insert(current_id, Arc::clone(&s));
            drop(p);

            let mut pd = player_data.lock().await;
            pd.insert(current_id, PlayerData { x: 50, y: 50 });
            drop(pd);

            s.listen().await?;

            let mut p = players.lock().await;
            let mut pd = player_data.lock().await;
            p.remove(&current_id);
            pd.remove(&current_id);

            for (_, socket) in p.iter() {
                packet::send_packet(packet::Packet::Disconnect { id: s.id }, &socket.socket)
                    .await?;
            }

            Ok::<_, io::Error>(())
        });

        current_id += 1;
    }
}
