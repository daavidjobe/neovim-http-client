extern crate neovim_lib;

use neovim_lib::{Neovim, NeovimApi, Session};

pub type EventHandlerResult<T> = Result<T, String>;

pub struct EventHandler {
    nvim: Neovim,
}

impl EventHandler {
    pub fn new() -> EventHandlerResult<Self> {
        let session = Session::new_parent().map_err(|e| format!("{}", e))?;
        let nvim = Neovim::new(session);

        Ok(EventHandler { nvim })
    }

    pub fn recv(&mut self) -> EventHandlerResult<()> {
        let receiver = self.nvim.session.start_event_loop_channel();
        Ok(for (event, _values) in receiver {
            self.nvim
                .command(&format!("echo \"Unknown command: {}\"", event))
                .map_err(|e| format!("{}", e))?
        })
    }
}
