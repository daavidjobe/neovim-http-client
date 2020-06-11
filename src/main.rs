mod eventhandler;
mod messages;
use crate::eventhandler::EventHandler;

fn main() {
    let mut event_handler = EventHandler::new().unwrap();
    let _ = event_handler.recv();
}
