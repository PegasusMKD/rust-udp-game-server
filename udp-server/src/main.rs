/**
*   This should be the UDP Server in which all calculations and logic for the game are done
*/

use std::io;

use tokio::net::UdpSocket;

use udp_server::game_server;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:8080").await?;
    let mut server = game_server::GameServer::new(socket);
    
    println!("Started UDP server on port 8080.");
    
    loop {
        server.process().await?;
    }
}
