use crate::geometry::*;

pub struct BulletInfo {
    position: Position,
    velocity: Velocity,
}

impl BulletInfo {
    
    pub fn update_position_in(&mut self, delta: u128) {
        self.position.x += (delta as f64 / 1000.0) * self.velocity.velocity_x;
        self.position.y += (delta as f64 / 1000.0) * self.velocity.velocity_y;
        self.position.z += (delta as f64 / 1000.0) * self.velocity.velocity_z;     
    }
}

pub struct BasicBullet {
    bullet_info: BulletInfo,
    speed: f64,
    damage: i16
}

impl BasicBullet {
    pub fn new(position: Position, direction: Direction) -> Self {
        let default_speed = 2.0;

        Self {
            bullet_info: BulletInfo { position, velocity: Velocity::new(direction, default_speed) },
            speed: default_speed,
            damage: 20
        }
    }
}


pub enum Bullet {
    Basic { bullet: BasicBullet },
}


pub trait TickUpdate {
    fn update_position(&mut self, delta: u128);
}

impl TickUpdate for Bullet {
    
    fn update_position(&mut self, delta: u128) {
        match self {
            Self::Basic { bullet } => bullet.bullet_info.update_position_in(delta) 
        }
    }

}
