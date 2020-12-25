//! Rewrite of the buffer_autohide.py script for Weechat
//!
//! The python script is perfectly fine, I am just eager to learn a bit of Rust ^-^

use weechat::hooks::{SignalCallback, SignalData, SignalHook};
use weechat::{plugin, Args, Plugin, ReturnCode, Weechat};

use callbacks::{BufferLineAdded, BufferSwitch};
use conf::Config;

mod callbacks;
mod conf;

struct BufferAutoHide {
    config: Config,
    buffer_switch: SignalHook,
    buffer_line_added: SignalHook,
}

impl Plugin for BufferAutoHide {
    fn init(_: &Weechat, _args: Args) -> Result<Self, ()> {
        let config = Config::new().expect("Error creating config");
        if let Err(e) = config.read() {
            Weechat::print(&format!(
                "Error reading the buffer_autohide config file {:?}",
                e
            ));
            return Err(());
        }

        Ok(Self {
            config,
            buffer_switch: SignalHook::new("buffer_switch", BufferSwitch::new())
                .expect("Could not register the buffer_switch callback"),
            buffer_line_added: SignalHook::new("buffer_line_added", BufferLineAdded::new())
                .expect("Could not register the buffer_line_added callback"),
        })
    }
}

plugin!(
    BufferAutoHide,
    name: "buffer_autohide",
    author: "Lara <lara@uwu.is>",
    description: "Automatically hide read buffers and unhide them on new activity",
    version: "0.0.1",
    license: "MIT"
);
