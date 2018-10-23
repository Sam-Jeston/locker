use std::sync::{Mutex, Arc};
use std::collections::HashMap;

// Register a new websocket connection
pub struct Register {
    pub ws: ws::Sender,
    pub channel_pointer: Arc<Mutex<HashMap<String, Box<ws::Sender>>>>,
}

impl ws::Handler for Register {
    // Here we add ourself to the vector of connections
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        println!("Register handler received a message: {}", msg);

        let owner_msg = String::from(msg.as_text().unwrap());
        let mut channel_ref = self.channel_pointer.lock().unwrap();

        let ws = self.ws.clone();
        channel_ref.insert(owner_msg, Box::new(ws));

        self.ws.send("")
    }
}
