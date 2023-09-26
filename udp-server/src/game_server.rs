use std::io;
use std::net::SocketAddr;

use prost::Message;
use tokio::net::UdpSocket;

use crate::game_info::*;

use crate::input_messages::game_event::*;
use crate::input_messages::*;
use crate::output_messages::UpdateGameEvent;
use crate::output_messages::update_game_event::UpdateEvent;

pub struct GameServer {
    pub socket: UdpSocket,
    pub game: GameInfo,
    
    read_buf: bytes::BytesMut,
    write_buf: bytes::BytesMut
}

impl GameServer {

    pub fn new(socket: UdpSocket) -> Self {
        GameServer { socket, game: GameInfo::default(), read_buf: bytes::BytesMut::new(), write_buf: bytes::BytesMut::new() }
    }

    pub async fn process(&mut self) -> io::Result<()> {
        let (_size, addr) = self.socket.recv_from(&mut self.read_buf).await?;
        let event = GameEvent::decode(&mut self.read_buf)?;
        self.read_buf.clear();
        // TODO: check if we should do this in a separate thread, as well as the update itself
        let update = self.process_event(event, addr);
        self.process_update(update).await?;
        self.write_buf.clear();
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
           self.socket.send_to(&self.write_buf, addr).await?;
        }

        Ok(())
    }
}
