//! Rewrite of the buffer_autohide.py script for Weechat
//!
//! The python script is perfectly fine, I am just eager to learn a bit of Rust ^-^

use std::{
    borrow::{Borrow, Cow},
    cell::RefCell,
    rc::Rc,
};

use weechat::{
    buffer::Buffer,
    hooks::{SignalCallback, SignalData, SignalHook},
    plugin, Args, Plugin, ReturnCode, Weechat,
};

use callbacks::{BufferLineAdded, BufferSwitch};
use conf::Config;

mod callbacks;
mod conf;

struct BufferAutoHide {
    /// A hook that listens to the `buffer_switch` signal
    buffer_switch: SignalHook,
    /// A hook that listens to the `buffer_line_added` signal
    buffer_line_added: SignalHook,
}

/// The inner state where we keep track of everything we need
#[derive(Clone)]
struct Inner<'a> {
    config: Rc<Config>,
    current_buffer: Rc<RefCell<Cow<'a, str>>>,
}

impl Plugin for BufferAutoHide {
    fn init(weechat: &Weechat, _args: Args) -> Result<Self, ()> {
        let config = Config::new().expect("Error creating config");
        if let Err(msg) = &config.read() {
            Weechat::print(&format!(
                "Error reading the buffer_autohide config file {:?}",
                msg
            ));
            return Err(());
        }

        // Convert to String to circumvent lifetime issues
        let buffer_name = String::from(weechat.current_buffer().full_name());
        let inner = Inner {
            config: Rc::new(config),
            current_buffer: Rc::new(RefCell::new(Cow::from(buffer_name))),
        };

        Ok(Self {
            buffer_switch: SignalHook::new("buffer_switch", BufferSwitch::new(inner.clone()))
                .expect("Could not register the buffer_switch callback"),
            buffer_line_added: SignalHook::new(
                "buffer_line_added",
                BufferLineAdded::new(inner.clone()),
            )
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
