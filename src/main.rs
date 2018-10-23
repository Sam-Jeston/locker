extern crate env_logger;
extern crate ws;

use std::sync::{Mutex, Arc};
use std::collections::HashMap;

// A WebSocket handler that routes connections to different boxed handlers by resource
struct Router {
    sender: ws::Sender,
    inner: Box<ws::Handler>,
    channel_pointer: Arc<Mutex<HashMap<String, Box<ws::Sender>>>>,
}

impl ws::Handler for Router {
    fn on_request(&mut self, req: &ws::Request) -> ws::Result<(ws::Response)> {
        // Clone the sender so that we can move it into the child handler
        let out = self.sender.clone();

        // /register -> Add to CONNECTIONS vector
        // /get_channels -> Simply get message channels from public key
        // /get_messages -> return encrypted messages based on predetermined key
        // /post_message -> post a message to a channel, find the client connection if it exists and
        // send them a message
        match req.resource() {
            "/get_channels" => self.inner = Box::new(Echo { ws: out }),
            "/register" => self.inner = Box::new(Register { ws: out, channel_pointer: self.channel_pointer.clone()}),
            "/get_messages" => {
                self.inner = Box::new(ChannelMessages {
                    ws: out,
                })
            },

            // Route to a data handler
            "/post_message" => {
                self.inner = Box::new(Poster {
                    ws: out,
                    channel_pointer: self.channel_pointer.clone(),
                })
            }
            // Use the default child handler, NotFound
            _ => (),
        }

        // Delegate to the child handler
        self.inner.on_request(req)
    }

    // Pass through any other methods that should be delegated to the child.
    fn on_shutdown(&mut self) {
        self.inner.on_shutdown()
    }

    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        self.inner.on_open(shake)
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        self.inner.on_message(msg)
    }

    // TODO: Remove ourself from the list of vector connections if it exists
    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        self.inner.on_close(code, reason)
    }

    fn on_error(&mut self, err: ws::Error) {
        self.inner.on_error(err);
    }
}

// This handler returns a 404 response to all handshake requests
struct NotFound;

impl ws::Handler for NotFound {
    fn on_request(&mut self, req: &ws::Request) -> ws::Result<(ws::Response)> {
        // This handler responds to all requests with a 404
        let mut res = ws::Response::from_request(req)?;
        res.set_status(404);
        res.set_reason("Not Found");
        Ok(res)
    }
}

// This handler simply echoes all messages back to the client
struct Echo {
    ws: ws::Sender,
}

impl ws::Handler for Echo {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        println!("Echo handler received a message: {}", msg);
        self.ws.send(msg)
    }
}

// This handler looks up the 30 most recent channel messages, and registers the client
struct ChannelMessages {
    ws: ws::Sender,
}

impl ws::Handler for ChannelMessages {
    // Here we add ourself to the vector of connections
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        println!("ChannelMessages handler received a message: {}", msg);

        // We will have the public key here, so we will actually add it here
        let connection = Connection {
            ws: self.ws.clone(),
            public_key: String::from("cat")
        };

        // TODO: Only push if the public key doesnt already exist
        self.ws.send(msg)
    }
}

struct Register {
    ws: ws::Sender,
    channel_pointer: Arc<Mutex<HashMap<String, Box<ws::Sender>>>>,
}

impl ws::Handler for Register {
    // Here we add ourself to the vector of connections
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        println!("Register handler received a message: {}", msg);

        let owner_msg = String::from(msg.as_text().unwrap());
        let mut channel_ref = self.channel_pointer.lock().unwrap();

        let ws = self.ws.clone();
        channel_ref.insert(owner_msg, Box::new(ws));

        self.ws.send("")
    }
}

// This handler sends some data to the client and then terminates the connection on the first
// message received, presumably confirming receipt of the data
struct Poster {
    ws: ws::Sender,
    channel_pointer: Arc<Mutex<HashMap<String, Box<ws::Sender>>>>,
}

struct Connection {
    ws: ws::Sender,
    public_key: String
}

impl ws::Handler for Poster {
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        // TODO: From the poster, we need to record the message, and do a send to the receipient
        // if the are currently connected
        println!("Data handler received a message: {}", msg);

        let owner_msg = "Actually the public key";
        let channel_ref = self.channel_pointer.lock().unwrap();

        match channel_ref.get(owner_msg) {
            Some(chan) => {
                chan.send("The new message");
                ()
            },
            None => ()
        };

        self.ws.send("")
    }
}

fn main() {
    env_logger::init();
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
