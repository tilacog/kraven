use std::process::Command;

use anyhow::{bail, Context, Result};

use crate::config::{ensure_profile_dir_exists, is_encrypted, resolve_profile_path};
use crate::profile::Profile;

pub fn run(profile_name: &str) -> Result<()> {
    ensure_profile_dir_exists()?;

    let profile_path = resolve_profile_path(profile_name)?;

    let editor = std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .unwrap_or_else(|_| "vi".to_string());

    let mut parts = editor.split_whitespace();
    let program = parts.next().unwrap_or("vi");
    let editor_args: Vec<&str> = parts.collect();

    let status = Command::new(program)
        .args(&editor_args)
        .arg(&profile_path)
        .status()
        .with_context(|| format!("Failed to launch editor: {editor}"))?;

    if !status.success() {
        bail!("Editor exited with non-zero status");
    }

    if !profile_path.exists() {
        println!("Profile '{profile_name}' was not created (no content saved).");
        return Ok(());
    }

    // Skip validation for encrypted files: re-loading would prompt for the passphrase again.
    if !is_encrypted(&profile_path) {
        if let Err(e) = Profile::load(profile_name, &profile_path) {
            eprintln!("Warning: Profile '{profile_name}' has errors:\n{e}");
            eprintln!("The file was saved, but you may want to fix these issues.");
            return Ok(());
        }
    }

    println!("Profile '{profile_name}' saved.");

    Ok(())
}
