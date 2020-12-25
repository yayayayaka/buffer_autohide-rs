use weechat::hooks::{SignalCallback, SignalData};
use weechat::{ReturnCode, Weechat};

pub(crate) struct BufferSwitch;

impl BufferSwitch {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl SignalCallback for BufferSwitch {
    fn callback<'a>(
        &mut self,
        _weechat: &Weechat,
        _signal_name: &str,
        _data: Option<SignalData<'a>>,
    ) -> ReturnCode {
        Weechat::print("Buffer switched");

        ReturnCode::Ok
    }
}

pub(crate) struct BufferLineAdded;

impl BufferLineAdded {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl SignalCallback for BufferLineAdded {
    fn callback<'a>(
        &mut self,
        _weechat: &Weechat,
        _signal_name: &str,
        _data: Option<SignalData<'a>>,
    ) -> ReturnCode {
        Weechat::print("Buffer line added");

        ReturnCode::Ok
    }
}
