use std::process::Command;

use anyhow::{bail, Context, Result};

use crate::config;

pub fn run(profile_name: &str) -> Result<()> {
    let (plain_path, gpg_path) = config::profile_paths(profile_name)?;

    if !gpg_path.exists() {
        bail!("Profile '{profile_name}' is not encrypted.");
    }

    if plain_path.exists() {
        bail!(
            "Plain profile '{profile_name}' already exists. Remove it first to avoid overwriting."
        );
    }

    let status = Command::new("gpg")
        .args(["--quiet", "--batch", "--decrypt", "--output"])
        .arg(&plain_path)
        .arg(&gpg_path)
        .status()
        .context("Failed to run gpg")?;

    if !status.success() {
        // Clean up partial output on failure
        let _ = std::fs::remove_file(&plain_path);
        bail!("gpg decryption failed");
    }

    std::fs::remove_file(&gpg_path)
        .with_context(|| format!("Failed to remove encrypted profile '{profile_name}'"))?;

    println!("Profile '{profile_name}' decrypted.");

    Ok(())
}
