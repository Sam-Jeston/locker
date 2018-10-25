use std::sync::{Mutex, Arc};
use std::collections::HashMap;

pub struct Register {
    pub ws: ws::Sender,
    pub channel_pointer: Arc<Mutex<HashMap<String, Box<ws::Sender>>>>,
}

#[derive(Serialize, Deserialize)]
struct RegistrationBody {
    public_key: String
}

impl ws::Handler for Register {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let raw_msg =  msg.as_text().unwrap();
        match serde_json::from_str::<RegistrationBody>(raw_msg) {
            Ok(parsed_msg) => {
                let mut channel_ref = self.channel_pointer.lock().unwrap();
                let ws = self.ws.clone();
                channel_ref.insert(parsed_msg.public_key, Box::new(ws));
                self.ws.send("{\"status\": \"connected\"}")
            },
            Err(_) => self.ws.close(ws::CloseCode::Error)
        }

    }
}
