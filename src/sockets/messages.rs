use database::messages::{create_message, get_messsages_for_channel};
use database::models::{MessageJson, ToJsonForm};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Message {
    pub ws: ws::Sender,
}

#[derive(Serialize, Deserialize)]
struct MessageBody {
    channel_id: i32,
}

impl ws::Handler for Message {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let raw_msg = msg.as_text().unwrap();
        match serde_json::from_str::<MessageBody>(raw_msg) {
            Ok(parsed_msg) => {
                // TODO: Page this data
                let messages = get_messsages_for_channel(parsed_msg.channel_id);
                let messages_json: Vec<MessageJson> =
                    messages.into_iter().map(|c| c.to_json_form()).collect();
                match serde_json::to_string(&messages_json) {
                    Ok(res) => self.ws.send(res),
                    Err(_) => self.ws.close(ws::CloseCode::Error),
                }
            }
            Err(_) => self.ws.close(ws::CloseCode::Error),
        }
    }
}

pub struct PostMessage {
    pub ws: ws::Sender,
    pub channel_pointer: Arc<Mutex<HashMap<String, Box<ws::Sender>>>>,
}

#[derive(Serialize, Deserialize)]
struct PostMessageBody {
    receiver_public_key: String,
    channel_id: i32,
    message: String,
    nonce: String,
}

impl ws::Handler for PostMessage {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let raw_msg = msg.as_text().unwrap();
        match serde_json::from_str::<PostMessageBody>(raw_msg) {
            Ok(parsed_msg) => {
                let new_message = create_message(
                    parsed_msg.channel_id,
                    &parsed_msg.message,
                    &parsed_msg.nonce,
                );
                let message_json: MessageJson = new_message.to_json_form();
                match serde_json::to_string(&message_json) {
                    Ok(res) => {
                        let channel_ref = self.channel_pointer.lock().unwrap();

                        match channel_ref.get(&parsed_msg.receiver_public_key) {
                            Some(chan) => {
                                let res_clone = res.clone();
                                chan.send(res_clone);
                            }
                            None => (),
                        };

                        self.ws.send(res)
                    }
                    Err(_) => self.ws.close(ws::CloseCode::Error),
                }
            }
            Err(_) => self.ws.close(ws::CloseCode::Error),
        }
    }
}
