#[derive(Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub struct Velocity {
    pub velocity_x: f64,
    pub velocity_y: f64,
    pub velocity_z: f64
}

pub struct Direction {
    direction_x: f64,
    direction_y: f64,
    direction_z: f64
}


impl Velocity {
    pub fn new(direction: Direction, speed: f64) -> Self {
        Self {
            velocity_x: direction.direction_x * speed,
            velocity_y: direction.direction_y * speed,
            velocity_z: direction.direction_z * speed
        }
    }
}


