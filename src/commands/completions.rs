use anyhow::Result;
use clap::CommandFactory;
use clap_complete::{generate, Shell};
use std::fs;
use std::io;

use crate::config::get_profile_dir;
use crate::Cli;

#[allow(clippy::unnecessary_wraps)] // Consistent return type with other commands
pub fn run(shell: Shell) -> Result<()> {
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();
    generate(shell, &mut cmd, name, &mut io::stdout());
    Ok(())
}

/// List profile names for shell completion (one per line).
pub fn list_profiles() -> Result<()> {
    let profile_dir = get_profile_dir()?;

    if !profile_dir.exists() {
        return Ok(());
    }

    let mut profiles: Vec<String> = fs::read_dir(&profile_dir)?
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            if !path.is_file() {
                return None;
            }
            let name = path.file_name()?.to_str()?;
            if name.starts_with('.') {
                return None;
            }
            Some(name.to_string())
        })
        .collect();

    profiles.sort();

    for profile in profiles {
        println!("{profile}");
    }

    Ok(())
}
