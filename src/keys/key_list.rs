use std::{fs::File, path::PathBuf};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{Deserialize, Serialize};

use struct_patch::traits::Patch as PatchTrait;
use struct_patch::Patch;

#[derive(Debug, PartialOrd, Clone, Copy, Serialize, Deserialize)]
pub struct P4KeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl P4KeyEvent {
    pub const fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }
}

pub fn key_match(ev: &KeyEvent, binding: P4KeyEvent) -> bool {
    ev.code == binding.code && ev.modifiers == binding.modifiers
}

impl PartialEq for P4KeyEvent {
    fn eq(&self, other: &Self) -> bool {
        let ev: KeyEvent = self.into();
        let other: KeyEvent = other.into();
        ev == other
    }
}

impl From<&P4KeyEvent> for KeyEvent {
    fn from(other: &P4KeyEvent) -> Self {
        Self::new(other.code, other.modifiers)
    }
}

#[derive(Clone, Patch)]
#[patch_derive(Deserialize)]
pub struct KeysList {
    pub tab_pending: P4KeyEvent,
    pub tab_submitted: P4KeyEvent,
    pub exit: P4KeyEvent,
    pub quit: P4KeyEvent,
    pub move_left: P4KeyEvent,
    pub move_right: P4KeyEvent,
    pub move_up: P4KeyEvent,
    pub move_down: P4KeyEvent,
    pub exit_popup: P4KeyEvent,
    pub open_help: P4KeyEvent,
}

impl Default for KeysList {
    fn default() -> Self {
        Self {
            tab_pending: P4KeyEvent::new(KeyCode::Char('1'), KeyModifiers::empty()),
            tab_submitted: P4KeyEvent::new(KeyCode::Char('2'), KeyModifiers::empty()),
            exit: P4KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
            quit: P4KeyEvent::new(KeyCode::Char('q'), KeyModifiers::empty()),
            move_left: P4KeyEvent::new(KeyCode::Left, KeyModifiers::empty()),
            move_right: P4KeyEvent::new(KeyCode::Right, KeyModifiers::empty()),
            move_up: P4KeyEvent::new(KeyCode::Up, KeyModifiers::empty()),
            move_down: P4KeyEvent::new(KeyCode::Down, KeyModifiers::empty()),
            exit_popup: P4KeyEvent::new(KeyCode::Esc, KeyModifiers::empty()),
            open_help: P4KeyEvent::new(KeyCode::Char('h'), KeyModifiers::empty()),
        }
    }
}

impl KeysList {
    pub fn init(file: PathBuf) -> Self {
        let mut keys_list = Self::default();
        if let Ok(f) = File::open(file) {
            if let Ok(patch) = ron::de::from_reader(f) {
                keys_list.apply(patch);
            }
        }
        keys_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_smoke() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r"
(
    move_down: Some(( code: Char('j'), modifiers: (bits: 2,),)),
)
"
        )
        .unwrap();

        let keys = KeysList::init(file.path().to_path_buf());

        assert_eq!(keys.move_right, KeysList::default().move_right);
        assert_eq!(
            keys.move_down,
            P4KeyEvent::new(KeyCode::Char('j'), KeyModifiers::CONTROL)
        );
    }
}
