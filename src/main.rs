#![allow(proc_macro_derive_resolution_fallback)]

extern crate env_logger;
extern crate ws;
extern crate serde_json;
extern crate dotenv;
extern crate chrono;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

mod sockets;
mod database;

use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use sockets::router::{Router, NotFound};
use database::{establish_connection};

fn main() {
    env_logger::init();

    // Ensure our DB is healthy
    establish_connection();

    let channel_ref = Arc::new(Mutex::new(HashMap::new()));

    // Listen on an address and call the closure for each connection
    if let Err(error) = ws::listen("127.0.0.1:3012", |out| {
        let channel_pointer = Arc::clone(&channel_ref);
        // Use our router as the handler to route the new connection
        Router {
            sender: out,
            inner: Box::new(NotFound),
            channel_pointer: channel_pointer,
        }
    }) {
        // Inform the user of failure
        println!("Failed to create WebSocket due to {:?}", error);
    }
}
