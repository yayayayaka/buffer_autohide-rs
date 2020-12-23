use std::collections::HashMap;
use std::hash::Hash;
use weechat::config::Config;
use weechat::{buffer::Buffer, plugin, Args, Plugin, Weechat};

struct BufferAutoHide {
    current_buffer: String,
    current_buffer_timer_hook: Option<i32>,
    keep_alive_buffers: HashMap<String, String>,
}

impl Plugin for BufferAutoHide {
    fn init(_: &Weechat, _: Args) -> Result<Self, ()> {
        Weechat::print("Hello from Rust");
        Ok(Self {
            current_buffer: "0x0".to_string(),
            current_buffer_timer_hook: None,
            keep_alive_buffers: HashMap::new(),
        })
    }
}

impl Drop for BufferAutoHide {
    fn drop(&mut self) {
        Weechat::print("Bye from Rust");
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
