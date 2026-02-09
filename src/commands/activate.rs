use std::process::Command;

use anyhow::{bail, Context, Result};

use crate::config::{resolve_profile_path, KRAVEN_ACTIVE};
use crate::profile::Profile;

pub fn run(profile_name: &str) -> Result<()> {
    if let Ok(active_profile) = std::env::var(KRAVEN_ACTIVE) {
        bail!(
            "Already in kraven session for profile '{active_profile}'.\n\
             Exit the current session first with 'exit' or Ctrl+D."
        );
    }

    let profile_path = resolve_profile_path(profile_name)?;
    let profile = Profile::load(profile_name, &profile_path)?;

    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());

    let mut cmd = Command::new(&shell);
    for (key, value) in &profile.vars {
        cmd.env(key, value);
    }
    cmd.env(KRAVEN_ACTIVE, profile_name);

    configure_prompt(&mut cmd, &shell, profile_name);

    let status = cmd
        .status()
        .with_context(|| format!("Failed to spawn shell: {shell}"))?;

    std::process::exit(status.code().unwrap_or(1));
}

/// Configures the shell prompt to show the active profile name.
///
/// Bash requires `PROMPT_COMMAND` because rc files override `PS1`.
/// Other shells (zsh, etc.) work with direct `PS1` modification.
fn configure_prompt(cmd: &mut Command, shell: &str, profile_name: &str) {
    let shell_name = std::path::Path::new(shell)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    if shell_name == "bash" {
        let existing = std::env::var("PROMPT_COMMAND").unwrap_or_default();
        let prefix_cmd = format!(r#"PS1="({profile_name}) ${{PS1#\({profile_name}\) }}""#);
        let prompt_cmd = if existing.is_empty() {
            prefix_cmd
        } else {
            format!("{prefix_cmd}; {existing}")
        };
        cmd.env("PROMPT_COMMAND", prompt_cmd);
    } else {
        let current_ps1 = std::env::var("PS1").unwrap_or_default();
        cmd.env("PS1", format!("({profile_name}) {current_ps1}"));
    }
}
