extern crate diesel;

use database::models::*;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use database::{establish_connection};
use super::schema::channels;

pub fn get_channels_for_client(client: &str) {
    use database::schema::channels::dsl::*;

    let connection = establish_connection();
    let results = channels
        .filter(creator.eq(&client))
        .or_filter(member.eq(&client))
        .load::<Channel>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} channels", results.len());
}

#[cfg(test)]
mod tests {
    use database::models::*;
    use diesel::prelude::*;
    use diesel::RunQueryDsl;
    use database::{establish_connection};
    use database::channels::get_channels_for_client;
    use database::schema::channels;

    #[test]
    fn returns_channels_when_client_is_creator() {
        let connection = establish_connection();

        let channels: Vec<NewChannel> = vec![
            NewChannel { creator: "foo", member: "bar" },
            NewChannel { creator: "faz", member: "bas" }
        ];

        diesel::insert_into(channels::table)
            .values(&channels)
            .get_result::<Channel>(&connection)
            .expect("Error saving new chanel");

        let results = get_channels_for_client("foo");
    }
}
