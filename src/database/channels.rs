extern crate diesel;

use database::models::*;
use diesel::prelude::*;

fn get_channels_for_client(client: String) {
    use database::schema::channels::dsl::*;

    let connection = establish_connection();
    let results = channels.filter(creator.eq(client))
        .load::<Channel>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} channels", results.len());
}
