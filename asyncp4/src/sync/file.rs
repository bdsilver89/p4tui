use std::path::PathBuf;

pub enum FileAction {
    Add,
    Edit,
    Delete,
    Branch,
    MoveAdd,
    MoveDelete,
    Integrate,
    Import,
    Purge,
    Archive,
}

pub enum FileType {
    Text,
    Binary,
    Symlink,
    Unicode,
    UTF8,
    UTF16,
    Apple,
    Resource,
}

pub struct File {
    exists_in_depot: bool,
    depot_path: PathBuf,
    local_path: PathBuf,
    head_action: FileAction,
    head_change: u32,
    head_revision: u32,
    head_filetype: FileType,
    have_revision: u32,
    work_revision: u32,
    action: FileAction,
    changelist: u32,
    filetype: FileType,
}

impl File {
    // pub fn new() -> Result<Self> {
    //
    // }
}
