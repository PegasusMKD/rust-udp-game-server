use std::net::SocketAddr;

use crate::bullet::BasicBullet;
use crate::entities::*;
use crate::input_messages;
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

    fn find_player_by_address(&self, addr: SocketAddr) -> Option<&Player> {
        let player_exists = self.players.iter().position(|player| player.server_info.addr == addr);
        if let Some(pos) = player_exists {
            let player = self.players.get(pos).unwrap();
            return Some(player);
        }

        None
 
    }

    pub fn shoot_bullet(&mut self, payload: input_messages::Shoot, addr: SocketAddr) -> Option<UpdateEvent> { 
        let player = self.find_player_by_address(addr);
        if player.is_none() {
            return None;
        }
        
        let position = player.unwrap().position.clone();
        let basic_bullet = BasicBullet::new(position, payload.direction.unwrap());
        let response = Some(UpdateEvent::CreateBullet(output_messages::CreateBullet::new(&basic_bullet.bullet_info)));
        self.bullets.push(Bullet::Basic { bullet: basic_bullet });
        response
    }

    pub fn game_tick(&mut self, delta: u128) -> Option<UpdateEvent> {
        let mut bullet_changes: Vec<output_messages::UpdateBulletPosition> = Vec::new();
        for bullet in self.bullets.iter_mut() {
            bullet_changes.push(bullet.update_position(delta));
        }

        if !bullet_changes.is_empty() {
            println!("Bullet data: {:?}\n", bullet_changes);
        }
        
        Some(UpdateEvent::UpdateAllBullets(
            output_messages::UpdateAllBullets {
                update_bullet_position: bullet_changes 
            }
        ))
    }

}

impl Default for GameInfo {
    
    fn default() -> Self {
        GameInfo { players: Vec::new(), bullets: Vec::new(), state: GameState {  } }
    }

}
