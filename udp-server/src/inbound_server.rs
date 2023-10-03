use std::collections::BTreeSet;

use std::io::{ self, ErrorKind };

use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use std::sync::Arc;

use prost::Message;

use crate::message_queue::*;
use crate::input_messages::*;

const BYTE_MAGIC_NUMBER: u8 = 0x2;

pub struct InboundServer {
    pub socket: Arc<Mutex<UdpSocket>>, // Shared
    pub message_queue: ConcurrentMessageQueue, // Shared 
}


impl InboundServer {

    pub fn new(socket: UdpSocket) -> Self {
        InboundServer { socket: Arc::new(Mutex::new(socket)),  message_queue: Arc::new(Mutex::new(BTreeSet::new())) }
    }
    
    pub async fn peek_latest_order(&self) -> i32 {
        if let Some(item) = self.message_queue.lock().await.last() {
            return item.order.clone();
        }
        
        return 0;
    }

    pub async fn wait_incoming_messages(&mut self) -> io::Result<()> {
        let mut buf = Vec::from([BYTE_MAGIC_NUMBER; 128]);
        let result = self.socket.lock().await.try_recv_from(&mut buf);
        
        match result {
            Err(ref e) => if e.raw_os_error().is_some_and(|err| err == 10054) || e.kind() == ErrorKind::WouldBlock { return Ok(()); },
            _ => ()
        }

        let (_size, addr) = result.unwrap();
        let clean_buf: Vec<u8> = buf.into_iter().filter(|b| *b != BYTE_MAGIC_NUMBER).collect();
        let event = GameEvent::decode(&mut clean_buf.as_slice())?;
        
        let last_order = self.peek_latest_order().await;
        self.message_queue.lock().await.insert(QueuedMessage { data: event.clone(), addr, order: last_order + 1 });
        Ok(())
    }
    
}
