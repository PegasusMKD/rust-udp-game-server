use std::io::{self, Write};

use prost::Message;
use tokio::{net::UdpSocket, time::Instant};

use udp_client::input_messages;
use input_messages::game_event::Event;

struct GameClient {
    socket: UdpSocket,
    rate_limit: i16,
    username: String,
    id: String,
    read_buf: bytes::BytesMut,
    write_buf: bytes::BytesMut
}

impl GameClient {

    pub fn new(socket: UdpSocket) -> Self {
        GameClient { 
            socket,
            rate_limit: 0,
            username: String::from("test"),
            id: uuid::Uuid::new_v4().to_string(),
            read_buf: bytes::BytesMut::new(),
            write_buf: bytes::BytesMut::new()
        }
    }

    pub fn configure(&mut self) {
        println!("Please select a rate limit per second (aka, frames per second you expect for this client)");
        print!("FPS: ");
        let _ = io::stdout().flush();
        let mut limit = String::new();
        io::stdin().read_line(&mut limit).expect("No rate limit...");
        self.rate_limit = limit.trim().parse().expect("woops parse");
        print!("\n");

        println!("Now enter username for this client");
        print!("Username: ");
        let _ = io::stdout().flush();
        let mut username = String::new();
        io::stdin().read_line(&mut username).expect("No rate limit...");
        self.username = username;
        print!("\n");
    }

    pub async fn join_lobby(&mut self) -> io::Result<()> {
        println!("Enter anything to join the lobby");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Lobby failed");
        let payload = input_messages::PlayerJoined { id: self.id.clone(), username: self.username.clone() }; 
        let body = input_messages::GameEvent { event: Some(Event::Joined(payload)) };
        self.write_buf.reserve(body.encoded_len());
        println!("Length is: {}", body.encoded_len());
        let _ = body.encode(&mut self.write_buf);
        self.socket.send(&self.write_buf).await?;
        self.write_buf.clear();
        println!("Joined lobby...");
        Ok(())
    }

    pub async fn simulate(&mut self) -> io::Result<()> {
        let start = Instant::now();
        let mut sent_messages = 0;
        // loop {
            let elapsed = start.elapsed();
            
            self.send_movement().await?;
            sent_messages += 1;
            if elapsed.as_secs() > 0 && elapsed.as_secs() % 30 == 0 {
                println!("Current message/second: {}", sent_messages / elapsed.as_secs());
            }
        println!("Uuuuhhhh");
        // }
        Ok(())
        
    }

    async fn send_movement(&mut self) -> io::Result<()> {
        let payload = input_messages::Move { distance_x: 10, distance_y: 10 };
        let body = input_messages::GameEvent { event: Some(Event::Move(payload)) };
        self.write_buf.reserve(body.encoded_len());
        let _ = body.encode(&mut self.write_buf);
        self.socket.send(&self.write_buf).await?;
        self.write_buf.clear();
        Ok(())
    }
}


#[tokio::main]
async fn main() -> io::Result<()> {
    let server_addr = "127.0.0.1:8080";

    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    println!("Created socket...");
    socket.connect(server_addr).await?;
    println!("Connected to server");

    let mut client = GameClient::new(socket);
    // client.configure();
    client.join_lobby().await?;
    client.simulate().await?;

    Ok(())
}
