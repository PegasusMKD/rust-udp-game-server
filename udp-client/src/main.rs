use std::io::{self, Write};

use rand::Rng;

use prost::Message;
use tokio::{net::UdpSocket, time::Instant};

use udp_client::input_messages::{self, Direction};
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

    pub fn configure_programmatically(&mut self, rate_limit: i16, username: String) {
        self.username = username;
        self.rate_limit = rate_limit;
    }

    pub async fn join_lobby(&mut self) -> io::Result<()> {
        println!("Enter anything to join the lobby");
        let input = String::new();
        // io::stdin().read_line(&mut input).expect("Lobby failed");
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
        let mut allow_print = true;
        let mut batch_messages = 0;
        let mut last_sec = 0;
        let mut second_messages = 0;
        let mut send_messages = true;
        let mut created_shot = false;
        loop {
            let elapsed = start.elapsed();
            if last_sec != elapsed.as_secs() {
                send_messages = true;
                second_messages = 0;
            }

            if send_messages {
                let number = {
                    let mut rng = rand::thread_rng();
                    rng.gen_range(0..101)
                };

                if number < 80 {
                    self.send_movement().await?;
                } else {
                    if created_shot {
                        continue;
                    }
                    created_shot = true;
                    self.send_shoot().await?;
                }
                sent_messages += 1;
                batch_messages += 1;
                second_messages += 1;
            }
            
            last_sec = elapsed.as_secs();

            if elapsed.as_secs() > 0 && elapsed.as_secs() % 6 != 0 {
                allow_print = true;
            }

            if elapsed.as_secs() > 0 && elapsed.as_secs() % 6 == 0 && allow_print {
                println!("Next log:");
                let time = chrono::Utc::now();
                println!("\t[{}] Current message/second: {}", time, sent_messages / elapsed.as_secs());
                println!("\t[{}] Total messages: {}", time, sent_messages);
                println!("\t[{}] Messages in this batch: {}", time, batch_messages);
                println!("\t[{}] Elapsed seconds: {}", time, elapsed.as_secs());
                allow_print = false;
                batch_messages = 0;
            }
        }
        
    }

    async fn send_shoot(&mut self) -> io::Result<()> {
        let payload = input_messages::Shoot { direction: Some(Direction { direction_x: 3.0, direction_y: 3.0, direction_z: 1.0 }) };
        let body = input_messages::GameEvent { event: Some(Event::Shoot(payload)) };
        self.write_buf.reserve(body.encoded_len());
        let _ = body.encode(&mut self.write_buf);
        self.socket.send(&self.write_buf).await?;
        self.write_buf.clear();
        Ok(())
    }

    async fn send_movement(&mut self) -> io::Result<()> {
        let payload = input_messages::Move { distance_x: 2.0, distance_y: 6.123456789 };
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
    let mut set = tokio::task::JoinSet::new();
    for idx in 0..5 {
        set.spawn(async move {
            let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
            println!("Created socket...");
            socket.connect(server_addr).await.unwrap();
            println!("Connected to server");
            
            let mut client = GameClient::new(socket);
            client.configure_programmatically(200, format!("test{}", idx.to_string()));
            client.join_lobby().await.unwrap();
            client.simulate().await.unwrap();
        });
        
    }

    set.join_next().await;
    Ok(())
}
