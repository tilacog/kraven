use std::process::Command;

use anyhow::{bail, Context, Result};

use crate::config;

pub fn run(profile_name: &str) -> Result<()> {
    let (plain_path, gpg_path) = config::profile_paths(profile_name)?;

    if gpg_path.exists() {
        bail!("Profile '{profile_name}' is already encrypted.");
    }

    if !plain_path.exists() {
        bail!("Profile '{profile_name}' does not exist.");
    }

    let status = Command::new("gpg")
        .args(["--quiet", "--batch", "--default-recipient-self", "--output"])
        .arg(&gpg_path)
        .arg("--encrypt")
        .arg(&plain_path)
        .status()
        .context("Failed to run gpg")?;

    if !status.success() {
        // Clean up partial output on failure
        let _ = std::fs::remove_file(&gpg_path);
        bail!("gpg encryption failed");
    }

    std::fs::remove_file(&plain_path)
        .with_context(|| format!("Failed to remove plain profile '{profile_name}'"))?;

    println!("Profile '{profile_name}' encrypted.");

    Ok(())
}
