use sockets::channels::{Channel, CreateChannel};
use sockets::messages::{Message, PostMessage};
use sockets::register::Register;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Router {
    pub sender: ws::Sender,
    pub inner: Box<ws::Handler>,
    pub channel_pointer: Arc<Mutex<HashMap<String, Box<ws::Sender>>>>,
}

impl ws::Handler for Router {
    fn on_request(&mut self, req: &ws::Request) -> ws::Result<(ws::Response)> {
        let out = self.sender.clone();

        match req.resource() {
            "/register" => {
                self.inner = Box::new(Register {
                    ws: out,
                    channel_pointer: self.channel_pointer.clone(),
                })
            }
            "/channels" => self.inner = Box::new(Channel { ws: out }),
            "/create_channel" => self.inner = Box::new(CreateChannel { ws: out }),
            "/post_message" => {
                self.inner = Box::new(PostMessage {
                    ws: out,
                    channel_pointer: self.channel_pointer.clone(),
                })
            }
            "/messages" => self.inner = Box::new(Message { ws: out }),
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

pub struct NotFound;

impl ws::Handler for NotFound {
    fn on_request(&mut self, req: &ws::Request) -> ws::Result<(ws::Response)> {
        let mut res = ws::Response::from_request(req)?;
        res.set_status(404);
        res.set_reason("Not Found");
        Ok(res)
    }
}
