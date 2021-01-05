use std::borrow::{Borrow, BorrowMut, Cow};
use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::ops::Deref;
use std::rc::Rc;

use weechat::buffer::Buffer;
use weechat::infolist::{Infolist, InfolistItem, InfolistVariable};
use weechat::{
    hooks::{SignalCallback, SignalData},
    ReturnCode, Weechat,
};

use crate::{conf::Config, Inner};

/// Struct that implements the SignalCallback for the `buffer_switch` signal.
pub(crate) struct BufferSwitch<'a> {
    inner: Inner<'a>,
}

impl<'a> BufferSwitch<'a> {
    pub(crate) fn new(inner: Inner<'a>) -> Self {
        Self { inner }
    }

    /// Check if passed buffer should be hidden.
    ///
    /// If configuration option ``hide_private`` is enabled,
    /// private buffers will become hidden as well.
    ///
    /// # Arguments
    ///
    /// * `weechat` - A reference to the Weechat struct
    /// * `buffer` - Buffer that should be hidden
    fn buffer_is_hidable(&self, weechat: &Weechat, buffer: &Buffer) -> bool {
        !(buffer.eq(&weechat.current_buffer())
            || buffer.full_name().starts_with("irc.server")
            || buffer.full_name().starts_with("core.")
            || (buffer
                .get_localvar("localvar_type")
                .eq(&Some(Cow::from("private")))
                && weechat
                    .get_plugin_option("hide_private")
                    .eq(&Some(Cow::from("off")))))
    }
}

impl<'a> SignalCallback for BufferSwitch<'a> {
    /// Hide the previous buffer when switching to another buffer
    fn callback(
        &mut self,
        weechat: &Weechat,
        _signal_name: &str,
        _data: Option<SignalData<'_>>,
    ) -> ReturnCode {
        let current_buffer = weechat.current_buffer();
        let previous_buffer = self
            .inner
            .current_buffer
            .replace(Cow::from(String::from(current_buffer.full_name()))); // ¯\_(ツ)_/¯

        if previous_buffer.eq(&current_buffer.full_name()) {
            return ReturnCode::Ok;
        }

        if let Some(buffer) = weechat.buffer_search("==", &previous_buffer) {
            if self.buffer_is_hidable(weechat, &buffer) {
                buffer.set_hidden();
            }
        }
        current_buffer.set_unhidden();

        ReturnCode::Ok
    }
}

/// Struct that implements the SignalCallback for the `buffer_line_added` signal.
pub(crate) struct BufferLineAdded<'a> {
    inner: Inner<'a>,
}

impl<'a> BufferLineAdded<'a> {
    /// TODO documentation
    pub(crate) fn new(inner: Inner<'a>) -> Self {
        Self { inner }
    }

    /// TODO documentation
    fn item_value_as_i32(&self, item: &mut InfolistItem, item_name: &str) -> i32 {
        match item.get(item_name) {
            Some(InfolistVariable::Integer(value)) => value,
            _ => 0,
        }
    }
}

impl SignalCallback for BufferLineAdded<'_> {
    /// TODO documentation
    fn callback(
        &mut self,
        weechat: &Weechat,
        _signal_name: &str,
        _data: Option<SignalData<'_>>,
    ) -> ReturnCode {
        let mut infolist = weechat.get_infolist("hotlist", None).unwrap();

        for mut item in infolist {
            let count_low = self.item_value_as_i32(item.borrow_mut(), "count_00");
            let count_message = self.item_value_as_i32(item.borrow_mut(), "count_01");
            let count_private = self.item_value_as_i32(item.borrow_mut(), "count_02");
            let count_highlight = self.item_value_as_i32(item.borrow_mut(), "count_03");

            if let Some(InfolistVariable::Buffer(buffer)) = &item.get("buffer_pointer") {
                if weechat.get_plugin_option("unhide_low") == Some(Cow::from("on")) && count_low > 0
                    || count_message > 0
                    || count_private > 0
                    || count_highlight > 0
                {
                    buffer.set_unhidden();
                }
            }
        }

        ReturnCode::Ok
    }
}
