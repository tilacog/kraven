use anyhow::{Context, Result};
use std::fs;

use crate::config::{get_profile_dir, KRAVEN_ACTIVE};

pub fn run() -> Result<()> {
    let profile_dir = get_profile_dir()?;

    if !profile_dir.exists() {
        println!("No profiles found. Profile directory does not exist yet.");
        println!("Use 'kraven edit <name>' to create your first profile.");
        return Ok(());
    }

    let mut profiles: Vec<String> = fs::read_dir(&profile_dir)
        .with_context(|| {
            format!(
                "Failed to read profile directory: {}",
                profile_dir.display()
            )
        })?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
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

    if profiles.is_empty() {
        println!("No profiles found.");
        println!("Use 'kraven edit <name>' to create your first profile.");
        return Ok(());
    }

    profiles.sort();

    // Check which profile is currently active
    let active = std::env::var(KRAVEN_ACTIVE).ok();

    for profile in profiles {
        if Some(&profile) == active.as_ref() {
            println!("{profile} (active)");
        } else {
            println!("{profile}");
        }
    }

    Ok(())
}
