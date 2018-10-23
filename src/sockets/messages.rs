use std::sync::{Mutex, Arc};
use std::collections::HashMap;
// TODO: get messages for a channel

// TODO: post a message against a channel and send it to the other participant if they are connected
pub struct PostMessage {
    pub ws: ws::Sender,
    pub channel_pointer: Arc<Mutex<HashMap<String, Box<ws::Sender>>>>,
}

impl ws::Handler for PostMessage {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        // TODO: From the poster, we need to record the message, and do a send to the receipient
        // if the are currently connected
        println!("Data handler received a message: {}", msg);

        let owner_msg = "Actually the public key";
        let channel_ref = self.channel_pointer.lock().unwrap();

        match channel_ref.get(owner_msg) {
            Some(chan) => {
                chan.send("The new message");
                ()
            },
            None => ()
        };

        self.ws.send("")
    }
}
