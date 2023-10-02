use std::net::SocketAddr;

use crate::networking::*;

use crate::geometry::*;

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
            position: Position { x: 0.0, y: 0.0, z: 0.0 }
        }
    }
}

pub struct GameState {

}


