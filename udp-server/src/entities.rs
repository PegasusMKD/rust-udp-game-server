use std::net::SocketAddr;

use crate::networking::*;

#[derive(Clone)]
pub struct Player {
    pub id: String,
    pub username: String,
    pub server_info: PlayerServerInfo,
    pub health: i32,
    pub position: Position
}

impl Player {
    
    pub fn new(id: String, username: String, addr: SocketAddr) -> Self {
        Player {
            id,
            username,
            server_info: PlayerServerInfo { addr },
            health: 100,
            position: Position { x: 0, y: 0, z: 0 }
        }
    }
}

#[derive(Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

pub struct GameState {

}


