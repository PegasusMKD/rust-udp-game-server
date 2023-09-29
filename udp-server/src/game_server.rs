use std::io::{self, ErrorKind};
use std::net::SocketAddr;

use prost::Message;
use tokio::net::UdpSocket;

use crate::{game_info::*, input_messages};

use bytes::BytesMut;

use crate::input_messages::game_event::*;
use crate::input_messages::*;
use crate::output_messages::UpdateGameEvent;
use crate::output_messages::update_game_event::UpdateEvent;

pub struct GameServer {
    pub socket: UdpSocket,
    pub game: GameInfo,
    
    write_buf: bytes::BytesMut,

    message_counter: i32
}

impl GameServer {

    pub fn new(socket: UdpSocket) -> Self {
        GameServer { socket, game: GameInfo::default(), write_buf: BytesMut::new(), message_counter: 0 }
    }

    pub async fn process(&mut self) -> io::Result<()> {
        let mut buf = Vec::from([0x0; 128]);
        let result = self.socket.try_recv_from(&mut buf);
        
        match result {
            Err(ref e) => if e.raw_os_error().is_some_and(|err| err == 10054) || e.kind() == ErrorKind::WouldBlock { return Ok(()); },
            _ => ()
        }

        let (_size, addr) = result.unwrap();
        let clean_buf: Vec<u8> = buf.into_iter().filter(|b| *b != 0x0).collect();
        let event = GameEvent::decode(&mut clean_buf.as_slice())?;
        
        // TODO: check if we should do this in a separate thread, as well as the update itself
        let update = self.process_event(event, addr);
        self.process_update(update).await?;
        self.write_buf.clear();
        self.message_counter += 1;

        if self.message_counter % 500 == 0 {
            println!("Processed {} messages...", self.message_counter);
        }
        Ok(())
    }


    fn process_event(&mut self, event: GameEvent, addr: SocketAddr) -> Option<UpdateEvent> {
        let ev = event.event.unwrap();
        match ev {
            Event::Joined(payload) => self.game.add_player(payload, addr),
            Event::Move(payload) => self.game.move_player(payload, addr),
            Event::Left(payload) => self.game.remove_player(payload, addr)
        }
    }

    async fn process_update(&mut self, event: Option<UpdateEvent>) -> io::Result<()> {
        if event == None {
            return Ok(());
        }

        let addrs = self.game.get_addresses();
        let ev = UpdateGameEvent { update_event: event }; 
        
        self.write_buf.reserve(ev.encoded_len());
        ev.encode(&mut self.write_buf)?;
        
        // TODO: Check if we should spawn this in a separate thread and put into an Arc mutex
        for addr in addrs {
            let result = self.socket.try_send_to(&self.write_buf, addr);
            if result.is_err() {
                self.game.remove_player(input_messages::PlayerLeft { id: String::new() }, addr);
            }
        }

        Ok(())
    }
}
