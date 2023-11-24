use rmp_serde::Serializer;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Packet {
    Init { id: u32 },
    Pos { id: u32, x: f32, y: f32 },
    Disconnect { id: u32 },
    Sprite { id: u32, sprite: u32 },
    Dir { id: u32, dir: f32, x_scale: f32 },
    Shot { id: u32 },
    Dmg { id: u32, amount: u32 },
    Death { id: u32 },
}

pub async fn send_packet(packet: Packet, socket: &tokio::net::TcpStream) -> io::Result<()> {
    let mut buf = Vec::new();
    packet.serialize(&mut Serializer::new(&mut buf)).unwrap();

    buf.splice(0..0, buf.len().to_le_bytes());

    socket.writable().await?;
    socket.try_write(buf.as_slice())?;

    println!("Sent packet: {packet:?}");
    Ok(())
}
