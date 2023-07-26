use std::borrow::Cow;

use unicode_truncate::UnicodeTruncateStr;
use unicode_width::UnicodeWidthStr;

use crate::keys::SharedKeyConfig;

pub mod symbol {
    pub const WHITESPACE: &str = "\u{00B7}"; //·
    pub const CHECKMARK: &str = "\u{2713}"; //✓
    pub const SPACE: &str = "\u{02FD}"; //˽
    pub const EMPTY_SPACE: &str = " ";
    pub const FOLDER_ICON_COLLAPSED: &str = "\u{25b8}"; //▸
    pub const FOLDER_ICON_EXPANDED: &str = "\u{25be}"; //▾
    pub const EMPTY_STR: &str = "";
    pub const ELLIPSIS: char = '\u{2026}'; // …
}

pub fn tab_pending(key_config: &SharedKeyConfig) -> String {
    format!(
        "Pending [{}]",
        key_config.get_hint(key_config.keys.tab_pending)
    )
}

pub fn tab_submitted(key_config: &SharedKeyConfig) -> String {
    format!(
        "Submitted [{}]",
        key_config.get_hint(key_config.keys.tab_submitted)
    )
}

pub fn tab_divider() -> String {
    " | ".to_string()
}

pub fn help_title(_kc: &SharedKeyConfig) -> String {
    "Help: all commands".to_string()
}

pub fn cmd_splitter() -> String {
    " ".to_string()
}

pub fn ellipsis_trim_start(s: &str, width: usize) -> Cow<str> {
    if s.width() <= width {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(format!(
            "[{}]{}",
            symbol::ELLIPSIS,
            s.unicode_truncate_start(width.saturating_sub(3 /* front indicator */))
                .0
        ))
    }
}

pub mod commands {
    use crate::components::CommandText;
    use crate::keys::SharedKeyConfig;

    static CMD_GROUP_GENERAL: &str = "-- General --";

    pub fn scroll(key_config: &SharedKeyConfig) -> CommandText {
        CommandText::new(
            format!(
                "Scroll [{}{}]",
                key_config.get_hint(key_config.keys.move_up),
                key_config.get_hint(key_config.keys.move_down)
            ),
            "scroll up or down in focused view",
            CMD_GROUP_GENERAL,
        )
    }
    pub fn close_popup(key_config: &SharedKeyConfig) -> CommandText {
        CommandText::new(
            format!(
                "Close [{}]",
                key_config.get_hint(key_config.keys.exit_popup),
            ),
            "close overlay (e.g commit, help)",
            CMD_GROUP_GENERAL,
        )
    }

    pub fn help_open(key_config: &SharedKeyConfig) -> CommandText {
        CommandText::new(
            format!("Help [{}]", key_config.get_hint(key_config.keys.open_help)),
            "open this help screen",
            CMD_GROUP_GENERAL,
        )
    }
}
