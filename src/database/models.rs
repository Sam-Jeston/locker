use diesel::sql_types::Timestamp;

#[derive(Queryable)]
pub struct Channel {
    pub id: i32,
    pub creator: String,
    pub member: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Queryable)]
pub struct Message {
    pub id: i32,
    pub channel_id: i32,
    pub message: String,
    pub nonce: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}
