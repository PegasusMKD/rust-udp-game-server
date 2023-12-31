pub mod input_messages {
    include!(concat!(env!("OUT_DIR"), "/input_messages.rs"));
}

pub mod output_messages {
    include!(concat!(env!("OUT_DIR"), "/output_messages.rs"));
}

pub mod networking;
pub mod game_server;
pub mod game_info;
pub mod entities;
pub mod bullet;
pub mod geometry;
pub mod message_queue;
pub mod inbound_server;
pub mod utility;
