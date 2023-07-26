use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum DiffLineType {
    None,
    Header,
    Add,
    Delete,
}

impl Default for DiffLineType {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct DiffLine {
    pub content: Box<str>,
    pub line_type: DiffLineType,
    pub position: DiffLinePosition,
}

#[derive(Clone, Copy, Default, Hash, Debug, PartialEq, Eq)]
pub struct DiffLinePosition {
    pub old_lineno: Option<u32>,
    pub new_lineno: Option<u32>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HunkHeader {
    pub old_start: u32,
    pub old_lines: u32,
    pub new_start: u32,
    pub new_lines: u32,
}

#[derive(Default, Clone, Hash, Debug)]
pub struct Hunk {
    pub header_hash: u64,
    pub lines: Vec<DiffLine>,
}

#[derive(Default, Clone, Hash, Debug)]
pub struct FileDiff {
    pub hunks: Vec<Hunk>,
    pub lines: usize,
    pub untracked: bool,
    pub sizes: (u64, u64),
    pub size_delta: i64,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffOptions {
    pub ignore_whitespace: bool,
    pub context: u32,
    pub interhunk_lines: u32,
}
