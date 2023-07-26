use anyhow::{anyhow, Result};
use std::{fs, path::PathBuf};

use clap::{crate_name, Arg, Command};

pub struct CliArgs {
    pub theme: PathBuf,
    pub cwd: PathBuf,
}

pub fn process_cmdline() -> Result<CliArgs> {
    let app = Command::new(crate_name!()).arg(
        Arg::new("theme")
            .help("Set the color theme (defaults to theme.ron)")
            .short('t')
            .long("theme")
            .value_name("THEME")
            .num_args(1),
    );

    let arg_matches = app.get_matches();

    let arg_theme = arg_matches
        .get_one::<String>("theme")
        .map_or_else(|| PathBuf::from("theme.ron"), PathBuf::from);

    let cwd = PathBuf::from(".");

    let theme = if get_app_config_path()?.join(&arg_theme).is_file() {
        get_app_config_path()?.join(arg_theme)
    } else {
        get_app_config_path()?.join("theme.ron")
    };

    Ok(CliArgs { theme, cwd })
}

pub fn get_app_config_path() -> Result<PathBuf> {
    let mut path = if cfg!(target_os = "macos") {
        dirs_next::home_dir().map(|h| h.join(".config"))
    } else {
        dirs_next::config_dir()
    }
    .ok_or_else(|| anyhow!("failed to find os config dir"))?;

    path.push("p4tui");
    fs::create_dir_all(&path)?;
    Ok(path)
}
