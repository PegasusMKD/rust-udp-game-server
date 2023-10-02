use std::net::SocketAddr;

use crate::bullet::BasicBullet;
use crate::entities::*;
use crate::geometry::*;
use crate::input_messages::*;
use crate::output_messages;

use crate::bullet::{Bullet, TickUpdate};


use output_messages::update_game_event::UpdateEvent;

pub struct GameInfo {
    players: Vec<Player>,
    bullets: Vec<Bullet>,
    state: GameState
}

impl GameInfo {

    pub fn get_addresses(&self) -> Vec<SocketAddr> { 
        self.players.iter().map(|player| player.server_info.addr).collect()
    }

    pub fn add_player(&mut self, data: PlayerJoined, addr: SocketAddr) -> Option<UpdateEvent> {
        let player_exists = self.players.iter().any(|player| player.server_info.addr == addr);
        if !player_exists {
            let player = Player::new(data.id.clone(), data.username.clone(), addr);
            self.players.push(player);
            return Some(UpdateEvent::AddedPlayer(output_messages::AddedPlayer { id: data.id, username: data.username }));
        }
        
        None
    }

    pub fn remove_player(&mut self, _data: PlayerLeft, addr: SocketAddr) -> Option<UpdateEvent> {
        let player_exists = self.players.iter().position(|player| player.server_info.addr == addr);
        if let Some(pos) = player_exists {
            let player = self.players.swap_remove(pos);
            return Some(UpdateEvent::RemovedPlayer(output_messages::RemovedPlayer { id: player.id }));
        } 

        None
    }

    pub fn move_player(&mut self, data: Move, addr: SocketAddr) -> Option<UpdateEvent> {
        let player_exists = self.players.iter().position(|player| player.server_info.addr == addr);
        if let Some(pos) = player_exists {
            let player = self.players.get_mut(pos).unwrap();
            player.position.x += data.distance_x;
            player.position.y += data.distance_y;
            return Some(UpdateEvent::ChangedPlayerPosition(
                output_messages::ChangedPlayerPosition 
                { 
                    x: player.position.x,
                    y: player.position.y,
                    z: player.position.z 
                }
            ));
        }

        None
    }

    pub fn shoot_bullet(&mut self, position: Position, direction: Direction) -> Option<UpdateEvent> {
        self.bullets.push(Bullet::Basic { bullet: BasicBullet::new(position, direction) });
        None // TODO: Add event which creates the bullet
    }

    pub fn game_tick(&mut self, delta: u128) -> Option<UpdateEvent> {
        for bullet in self.bullets.iter_mut() {
            bullet.update_position(delta);
        }
        None // TODO: Add event which updates bullet position
    }

}

impl Default for GameInfo {
    
    fn default() -> Self {
        GameInfo { players: Vec::new(), bullets: Vec::new(), state: GameState {  } }
    }

}
