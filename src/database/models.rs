use chrono::NaiveDateTime ;
use super::schema::channels;
use super::schema::messages;

pub trait ToJsonForm<T> {
    fn to_json_form(&self) -> T;
}

#[derive(Queryable)]
pub struct Channel {
    pub id: i32,
    pub creator: String,
    pub member: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct ChannelJson {
    pub id: i32,
    pub creator: String,
    pub member: String,
    pub created_at: String,
    pub updated_at: String,
}

impl ToJsonForm<ChannelJson> for Channel {
    fn to_json_form(&self) -> ChannelJson {
        let created_at = self.created_at.to_string();
        let updated_at = self.created_at.to_string();
        ChannelJson {
            id: self.id,
            creator: self.creator.clone(),
            member: self.member.clone(),
            created_at: created_at,
            updated_at: updated_at
        }
    }
}

#[derive(Insertable)]
#[table_name="channels"]
pub struct NewChannel<'a> {
    pub creator: &'a str,
    pub member: &'a str,
}

#[derive(Queryable)]
pub struct Message {
    pub id: i32,
    pub channel_id: i32,
    pub message: String,
    pub nonce: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="messages"]
pub struct NewMessage<'a> {
    pub channel_id: &'a i32,
    pub message: &'a str,
    pub nonce: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct MessageJson {
    pub id: i32,
    pub channel_id: i32,
    pub message: String,
    pub nonce: String,
    pub created_at: String,
    pub updated_at: String,
}

impl ToJsonForm<MessageJson> for Message {
    fn to_json_form(&self) -> MessageJson {
        let created_at = self.created_at.to_string();
        let updated_at = self.created_at.to_string();
        MessageJson {
            id: self.id,
            channel_id: self.channel_id,
            message: self.message.clone(),
            nonce: self.nonce.clone(),
            created_at: created_at,
            updated_at: updated_at
        }
    }
}
