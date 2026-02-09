use std::collections::BTreeMap;
use std::fs;

use anyhow::{Context, Result};

use crate::config::{get_profile_dir, KRAVEN_ACTIVE};

pub fn run() -> Result<()> {
    let profile_dir = get_profile_dir()?;

    if !profile_dir.exists() {
        println!("No profiles found. Profile directory does not exist yet.");
        println!("Use 'kraven edit <name>' to create your first profile.");
        return Ok(());
    }

    let mut profiles = BTreeMap::<String, bool>::new();

    for entry in fs::read_dir(&profile_dir)
        .with_context(|| {
            format!(
                "Failed to read profile directory: {}",
                profile_dir.display()
            )
        })?
        .flatten()
    {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(file_name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if file_name.starts_with('.') {
            continue;
        }
        let (name, encrypted) = match file_name.strip_suffix(".gpg") {
            Some(n) => (n, true),
            None => (file_name, false),
        };
        profiles
            .entry(name.to_string())
            .and_modify(|e| *e |= encrypted)
            .or_insert(encrypted);
    }

    if profiles.is_empty() {
        println!("No profiles found.");
        println!("Use 'kraven edit <name>' to create your first profile.");
        return Ok(());
    }

    let active = std::env::var(KRAVEN_ACTIVE).ok();

    for (name, encrypted) in &profiles {
        let mut indicators = Vec::new();
        if active.as_deref() == Some(name.as_str()) {
            indicators.push("active");
        }
        if *encrypted {
            indicators.push("encrypted");
        }
        if indicators.is_empty() {
            println!("{name}");
        } else {
            println!("{name} ({})", indicators.join(", "));
        }
    }

    Ok(())
}
