use std::{process::Command, str};

use crate::{Error, Result};

use regex::Regex;

pub enum ChangelistStatus {
    None,
    Pending,
    Submitted,
}

pub struct Changelist {
    changelist: u32,
    // files: Vec<File>,
    status: ChangelistStatus,
}

impl Changelist {
    pub fn new() -> Self {
        Self {
            changelist: 0,
            // files: Vec::new(),
            status: ChangelistStatus::None,
        }
    }

    // pub fn file<'a>(&'a mut self, f: File) -> &'a mut Changelist {
    //     self.files.push(f);
    //     self
    // }
}

pub fn get_pending_changelists(user: Option<String>, client: Option<String>) -> Result<Vec<u32>> {
    get_changelists_impl(user, client, ChangelistStatus::Pending)
}

pub fn get_submitted_changelists(user: Option<String>, client: Option<String>) -> Result<Vec<u32>> {
    get_changelists_impl(user, client, ChangelistStatus::Submitted)
}

fn get_changelists_impl(
    user: Option<String>,
    client: Option<String>,
    status: ChangelistStatus,
) -> Result<Vec<u32>> {
    let mut cmd = Command::new("p4");
    cmd.arg("changes");
    if let Some(u) = user {
        cmd.arg("-u");
        cmd.arg(u);
    }
    if let Some(c) = client {
        cmd.arg("-c");
        cmd.arg(c);
    }
    match status {
        ChangelistStatus::Pending => {
            cmd.arg("-s");
            cmd.arg("pending");
        }
        ChangelistStatus::Submitted => {
            cmd.arg("-s");
            cmd.arg("submitted");
        }
        _ => {}
    }

    let output = cmd.output().map_err(|e| Error::from(e))?;

    let mut result = Vec::new();
    for line in str::from_utf8(output.stdout.as_slice())
        .unwrap()
        // FIXME: replace unwrap with map err
        .split('\n')
        .filter(|l| !l.is_empty())
    {
        // FIXME: replace unwrap with map err
        let re = Regex::new(r"Change ([0-9]+)").unwrap();
        if let Some(caps) = re.captures(line) {
            let cl = String::from(&caps[1]).parse::<u32>().unwrap();
            result.push(cl);
        }
    }

    Ok(result)
}
