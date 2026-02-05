use anyhow::{bail, Context, Result};
use std::process::Command;

use crate::config::{get_profile_path, KRAVEN_ACTIVE};
use crate::profile::Profile;

pub fn run(profile_name: &str) -> Result<()> {
    // Prevent nested sessions
    if let Ok(active_profile) = std::env::var(KRAVEN_ACTIVE) {
        bail!(
            "Already in kraven session for profile '{active_profile}'.\n\
             Exit the current session first with 'exit' or Ctrl+D."
        );
    }

    let profile_path = get_profile_path(profile_name)?;
    let profile = Profile::load(profile_name, &profile_path)?;

    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());

    let mut cmd = Command::new(&shell);

    // Inject all parsed environment variables
    for (key, value) in &profile.vars {
        cmd.env(key, value);
    }

    // Mark this session
    cmd.env(KRAVEN_ACTIVE, profile_name);

    // Modify prompt to show active profile
    // Different shells need different approaches since rc files override PS1
    let shell_name = std::path::Path::new(&shell)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    if shell_name == "bash" {
        // PROMPT_COMMAND runs before each prompt, allowing us to modify PS1
        // after .bashrc has set it
        let existing = std::env::var("PROMPT_COMMAND").unwrap_or_default();
        let prefix_cmd = format!(r#"PS1="({profile_name}) ${{PS1#\({profile_name}\) }}""#);
        let new_prompt_cmd = if existing.is_empty() {
            prefix_cmd
        } else {
            format!("{prefix_cmd}; {existing}")
        };
        cmd.env("PROMPT_COMMAND", new_prompt_cmd);
    } else {
        // For zsh and other shells, set PS1 directly
        let current_ps1 = std::env::var("PS1").unwrap_or_default();
        cmd.env("PS1", format!("({profile_name}) {current_ps1}"));
    }

    // Run interactively
    let status = cmd
        .status()
        .with_context(|| format!("Failed to spawn shell: {shell}"))?;

    std::process::exit(status.code().unwrap_or(1));
}
