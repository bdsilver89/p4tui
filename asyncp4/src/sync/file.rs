use crate::{Error, Result};
use regex::Regex;
use std::{path::PathBuf, process::Command, str};

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

pub fn get_files(changelist: u32) -> Result<Vec<String>> {
    let mut cmd = Command::new("p4");
    cmd.args(["describe", changelist.to_string().as_str()]);

    let output = cmd.output().map_err(|e| Error::from(e))?;

    let mut result = Vec::new();
    for line in str::from_utf8(output.stdout.as_slice())
        .unwrap()
        // FIXME: replace unwrap with map err
        .split('\n')
        .filter(|l| !l.is_empty())
    {
        // FIXME: replace unwrap with map err
        let re = Regex::new(r"\.\.\.s+(\S+)#([0-9]+)\s+(\S+)").unwrap();

        if let Some(caps) = re.captures(line) {
            let file_line = String::from(&caps[0]);
            result.push(file_line);
        }
    }

    Ok(result)
}
