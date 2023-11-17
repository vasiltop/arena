#![warn(clippy::pedantic)]

mod packet;
mod socket;

use std::{
    io,
    sync::{Arc, Mutex},
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").await?;

    let mut queue = Arc::new(Mutex::new(Vec::new()));
    loop {
        let (socket, _) = listener.accept().await?;

        let queue = queue.clone();
        tokio::spawn(async move {
            let s = socket::Socket { socket, queue };
            s.listen().await;
        });
    }
}
