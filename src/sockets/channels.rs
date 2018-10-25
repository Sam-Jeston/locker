use database::channels::{create_channel, get_channels_for_client};
use database::models::{ChannelJson, ToJsonForm};

pub struct Channel {
    pub ws: ws::Sender,
}

#[derive(Serialize, Deserialize)]
struct ChannelBody {
    public_key: String,
}

impl ws::Handler for Channel {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let raw_msg = msg.as_text().unwrap();
        match serde_json::from_str::<ChannelBody>(raw_msg) {
            Ok(parsed_msg) => {
                let channels = get_channels_for_client(&parsed_msg.public_key);
                let channels_json: Vec<ChannelJson> =
                    channels.into_iter().map(|c| c.to_json_form()).collect();
                match serde_json::to_string(&channels_json) {
                    Ok(res) => self.ws.send(res),
                    Err(_) => self.ws.close(ws::CloseCode::Error),
                }
            }
            Err(_) => self.ws.close(ws::CloseCode::Error),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct CreateChannelBody {
    creator: String,
    member: String,
}

pub struct CreateChannel {
    pub ws: ws::Sender,
}

impl ws::Handler for CreateChannel {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let raw_msg = msg.as_text().unwrap();
        match serde_json::from_str::<CreateChannelBody>(raw_msg) {
            Ok(parsed_msg) => {
                let new_channel = create_channel(&parsed_msg.creator, &parsed_msg.member);
                let channel_json: ChannelJson = new_channel.to_json_form();
                match serde_json::to_string(&channel_json) {
                    Ok(res) => self.ws.send(res),
                    Err(_) => self.ws.close(ws::CloseCode::Error),
                }
            }
            Err(_) => self.ws.close(ws::CloseCode::Error),
        }
    }
}
