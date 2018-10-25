use chrono::NaiveDateTime ;
use super::schema::channels;
use super::schema::messages;

#[derive(Queryable)]
pub struct Channel {
    pub id: i32,
    pub creator: String,
    pub member: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
