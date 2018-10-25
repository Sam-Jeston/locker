#[macro_use]
pub mod schema;
pub mod models;
pub mod channels;
pub mod messages;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn truncate_tables() -> PgConnection {
        let conn = establish_connection();
        conn.execute("TRUNCATE TABLE messages CASCADE").unwrap();
        conn.execute("TRUNCATE TABLE channels CASCADE").unwrap();
        conn
    }
}
