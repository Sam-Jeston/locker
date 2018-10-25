extern crate diesel;

use database::establish_connection;
use database::models::*;
use database::schema::channels;
use database::schema::channels::dsl::*;
use diesel::prelude::*;
use diesel::RunQueryDsl;

pub fn get_channels_for_client(client: &str) -> Vec<Channel> {
    let connection = establish_connection();
    channels
        .filter(creator.eq(&client))
        .or_filter(member.eq(&client))
        .load::<Channel>(&connection)
        .expect("Error loading channels")
}

pub fn create_channel(crt: &str, mem: &str) -> Channel {
    let connection = establish_connection();
    let channel = NewChannel {
        creator: crt,
        member: mem,
    };

    diesel::insert_into(channels::table)
        .values(&channel)
        .get_result::<Channel>(&connection)
        .expect("Error saving new channel")
}

#[cfg(test)]
mod tests {
    use super::*;
    use database::tests::truncate_tables;

    #[test]
    fn returns_channels_when_client_is_creator() {
        let connection = truncate_tables();

        let new_channels: Vec<NewChannel> = vec![
            NewChannel {
                creator: "foo",
                member: "bar",
            },
            NewChannel {
                creator: "faz",
                member: "bas",
            },
        ];

        diesel::insert_into(channels::table)
            .values(&new_channels)
            .get_result::<Channel>(&connection)
            .expect("Error saving new channel");

        let results = get_channels_for_client("foo");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].creator, "foo");
    }

    #[test]
    fn returns_channels_when_client_is_member() {
        let connection = truncate_tables();

        let new_channels: Vec<NewChannel> = vec![
            NewChannel {
                creator: "foo",
                member: "bar",
            },
            NewChannel {
                creator: "faz",
                member: "bas",
            },
        ];

        diesel::insert_into(channels::table)
            .values(&new_channels)
            .get_result::<Channel>(&connection)
            .expect("Error saving new channel");

        let results = get_channels_for_client("bas");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].creator, "faz");
    }

    #[test]
    fn successfully_creates_a_channel() {
        truncate_tables();

        create_channel("foo", "bar");
        let results = get_channels_for_client("foo");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].member, "bar");
    }
}
