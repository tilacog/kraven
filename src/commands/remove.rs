use anyhow::{bail, Context, Result};
use std::fs;
use std::io::{self, Write};

use crate::config::{get_profile_path, KRAVEN_ACTIVE};

pub fn run(profile_name: &str, force: bool) -> Result<()> {
    let profile_path = get_profile_path(profile_name)?;

    if !profile_path.exists() {
        bail!("Profile '{profile_name}' does not exist.");
    }

    // Warn if removing the currently active profile
    if let Ok(active) = std::env::var(KRAVEN_ACTIVE) {
        if active == profile_name {
            eprintln!("Warning: '{profile_name}' is the currently active profile.");
        }
    }

    if !force {
        print!("Remove profile '{profile_name}'? [y/N] ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim().to_lowercase();
        if input != "y" && input != "yes" {
            println!("Aborted.");
            return Ok(());
        }
    }

    fs::remove_file(&profile_path)
        .with_context(|| format!("Failed to remove profile '{profile_name}'"))?;

    println!("Profile '{profile_name}' removed.");

    Ok(())
}
