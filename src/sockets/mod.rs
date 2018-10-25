use database::channels::{create_channel, get_channels_for_client};
use database::messages::{create_message, get_messsages_for_channel};
use database::models::{ChannelJson, MessageJson, ToJsonForm};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Router {
    pub sender: ws::Sender,
    pub channel_pointer: Arc<Mutex<HashMap<String, Box<ws::Sender>>>>,
    pub public_key: String,
}

#[derive(Serialize, Deserialize)]
struct RegistrationBody {
    public_key: String,
}

#[derive(Serialize, Deserialize)]
struct CreateChannelBody {
    creator: String,
    member: String,
}

#[derive(Serialize, Deserialize)]
struct MessageBody {
    channel_id: i32,
}

#[derive(Serialize, Deserialize)]
struct PostMessageBody {
    receiver_public_key: String,
    channel_id: i32,
    message: String,
    nonce: String,
}

impl ws::Handler for Router {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let raw_msg = msg.as_text().unwrap();
        // This approach is a little primative. Match on the incoming message, if it parses correctly,
        // do the behaviour for that action type. Its okay for now
        let routing = (
            serde_json::from_str::<RegistrationBody>(raw_msg),
            serde_json::from_str::<CreateChannelBody>(raw_msg),
            serde_json::from_str::<MessageBody>(raw_msg),
            serde_json::from_str::<PostMessageBody>(raw_msg),
        );

        match routing {
            // Registration: Register the ws in our HashMap and return the channels to the user
            (Ok(parsed_msg), _, _, _) => {
                let mut channel_ref = self.channel_pointer.lock().unwrap();
                let ws = self.sender.clone();
                channel_ref.insert(parsed_msg.public_key.clone(), Box::new(ws));
                self.public_key = parsed_msg.public_key.clone();

                let channels = get_channels_for_client(&parsed_msg.public_key);
                let channels_json: Vec<ChannelJson> =
                    channels.into_iter().map(|c| c.to_json_form()).collect();
                match serde_json::to_string(&channels_json) {
                    Ok(res) => self.sender.send(res),
                    Err(_) => self.sender.close(ws::CloseCode::Error),
                }
            }
            // Channel Creation: Create a new channel with the desired keys
            (_, Ok(parsed_msg), _, _) => {
                let new_channel = create_channel(&parsed_msg.creator, &parsed_msg.member);
                let channel_json: ChannelJson = new_channel.to_json_form();
                match serde_json::to_string(&channel_json) {
                    Ok(res) => self.sender.send(res),
                    Err(_) => self.sender.close(ws::CloseCode::Error),
                }
            }
            // Get Messages: Return messages for a channel
            (_, _, Ok(parsed_msg), _) => {
                // TODO: Page this data
                let messages = get_messsages_for_channel(parsed_msg.channel_id);
                let messages_json: Vec<MessageJson> =
                    messages.into_iter().map(|c| c.to_json_form()).collect();
                match serde_json::to_string(&messages_json) {
                    Ok(res) => self.sender.send(res),
                    Err(_) => self.sender.close(ws::CloseCode::Error),
                }
            }
            // Post Message: Record a new message against a channel. Send this message to the receiver
            // if they are currently connected
            (_, _, _, Ok(parsed_msg)) => {
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

                        self.sender.send(res)
                    }
                    Err(_) => self.sender.close(ws::CloseCode::Error),
                }
            }
            _ => self.sender.send(""),
        }
    }

    // TODO: Remove ws connection from the list of vector connections if it exists
    // fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
    // }
}
