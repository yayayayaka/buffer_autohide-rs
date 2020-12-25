use weechat::config;

config!(
    "buffer_autohide",
    Section look {
        hide_inactive: bool {
            "Hide inactive buffers (default: \"off\")",
            false,
        },
        hide_private: bool {
            "Hide private buffers (default: \"off\")",
            false,
        },
        unhide_low: bool {
            "Unhide a buffer when a low priority message (like JOIN,
        PART, etc.) has been received (default: \"off\")",
            false,
        },
        exemptions: String {
            "An enumeration of buffers that should not become hidden (default: \"\")",
            "",
        },
        keep_open: bool {
            "Keep a buffer open for a short amount of time (default: \"off\")",
            false,
        },
        keep_open_timeout: Integer {
            "Timeout in milliseconds for how long a selected buffer should be kept around (default: \"60 * 1000\")",
            60000,
            0..2147483647, // passing `std::i32::MAX` doesn't work...
        },
    },
);
