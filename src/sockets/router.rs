use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use sockets::register::Register;
use sockets::messages::PostMessage;

// A WebSocket handler that routes connections to different boxed handlers by resource
pub struct Router {
    pub sender: ws::Sender,
    pub inner: Box<ws::Handler>,
    pub channel_pointer: Arc<Mutex<HashMap<String, Box<ws::Sender>>>>,
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
            "/register" => self.inner = Box::new(Register { ws: out, channel_pointer: self.channel_pointer.clone()}),
            "/post_message" => {
                self.inner = Box::new(PostMessage {
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
pub struct NotFound;

impl ws::Handler for NotFound {
    fn on_request(&mut self, req: &ws::Request) -> ws::Result<(ws::Response)> {
        // This handler responds to all requests with a 404
        let mut res = ws::Response::from_request(req)?;
        res.set_status(404);
        res.set_reason("Not Found");
        Ok(res)
    }
}
