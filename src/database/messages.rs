extern crate diesel;

use database::establish_connection;
use database::models::*;
use database::schema::messages;
use database::schema::messages::dsl::*;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub fn get_messsages_for_channel(chan_id: i32) -> Vec<Message> {
    let connection = establish_connection();
    messages
        .filter(channel_id.eq(&chan_id))
        .load::<Message>(&connection)
        .expect("Error loading messages")
}

pub fn create_message(chan_id: i32, new_message: &str, new_nonce: &str) -> Message {
    let connection = establish_connection();
    let msg = NewMessage {
        channel_id: &chan_id,
        message: new_message,
        nonce: new_nonce,
    };

    diesel::insert_into(messages::table)
        .values(&msg)
        .get_result::<Message>(&connection)
        .expect("Error saving new message")
}

#[cfg(test)]
mod tests {
    use super::*;
    use database::channels::create_channel;
    use database::tests::truncate_tables;

    #[test]
    fn creates_a_new_message_for_a_channel() {
        truncate_tables();

        let chan = create_channel("foo", "bar");
        let msg = create_message(chan.id, "message", "nonce");

        assert_eq!(msg.channel_id, chan.id);
        assert_eq!(msg.message, "message");
        assert_eq!(msg.nonce, "nonce");
    }

    #[test]
    fn returns_all_messages_for_a_channel() {
        truncate_tables();

        let chan = create_channel("foo", "bar");
        let chan2 = create_channel("baz", "bar");
        create_message(chan.id, "message", "nonce");
        create_message(chan2.id, "message2", "nonce2");

        let msgs = get_messsages_for_channel(chan.id);

        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].message, "message");
    }
}
