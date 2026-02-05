use anyhow::{bail, Context, Result};
use std::path::PathBuf;

/// Environment variable marking an active kraven session.
pub const KRAVEN_ACTIVE: &str = "KRAVEN_ACTIVE";

const ENV_PROFILE_DIR: &str = "KRAVEN_PROFILE_DIR";
const DEFAULT_PROFILE_SUBDIR: &str = "kraven";

/// Validates that a profile name is safe (no path traversal or shell injection).
/// Allowed: alphanumeric, underscore, hyphen, and dot (but not `.` or `..` alone).
fn validate_profile_name(name: &str) -> Result<()> {
    if name.is_empty() {
        bail!("Profile name cannot be empty");
    }

    if name == "." || name == ".." {
        bail!("Invalid profile name: '{name}'");
    }

    if name.starts_with('-') {
        bail!("Profile name cannot start with '-': '{name}'");
    }

    let is_valid_char = |c: char| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.');
    if let Some(c) = name.chars().find(|c| !is_valid_char(*c)) {
        bail!("Profile name contains invalid character '{c}': '{name}'");
    }

    Ok(())
}

/// Returns the directory where profiles are stored.
pub fn get_profile_dir() -> Result<PathBuf> {
    if let Ok(custom_dir) = std::env::var(ENV_PROFILE_DIR) {
        return Ok(PathBuf::from(custom_dir));
    }
    let config_dir = dirs::config_dir().context("Could not determine config directory")?;
    Ok(config_dir.join(DEFAULT_PROFILE_SUBDIR))
}

/// Returns the full path to a profile file, validating the profile name.
pub fn get_profile_path(profile_name: &str) -> Result<PathBuf> {
    validate_profile_name(profile_name)?;
    let profile_dir = get_profile_dir()?;
    Ok(profile_dir.join(profile_name))
}

/// Ensures the profile directory exists, creating it if necessary.
pub fn ensure_profile_dir_exists() -> Result<PathBuf> {
    let profile_dir = get_profile_dir()?;
    if !profile_dir.exists() {
        std::fs::create_dir_all(&profile_dir).with_context(|| {
            format!(
                "Failed to create profile directory: {}",
                profile_dir.display()
            )
        })?;
    }
    Ok(profile_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_profile_names() {
        assert!(validate_profile_name("dev").is_ok());
        assert!(validate_profile_name("prod-1").is_ok());
        assert!(validate_profile_name("my_profile").is_ok());
        assert!(validate_profile_name("v1.2.3").is_ok());
        assert!(validate_profile_name("AWS_PROD").is_ok());
    }

    #[test]
    fn test_empty_profile_name() {
        assert!(validate_profile_name("").is_err());
    }

    #[test]
    fn test_path_traversal_blocked() {
        assert!(validate_profile_name(".").is_err());
        assert!(validate_profile_name("..").is_err());
        assert!(validate_profile_name("../etc").is_err());
        assert!(validate_profile_name("foo/bar").is_err());
    }

    #[test]
    fn test_leading_dash_blocked() {
        assert!(validate_profile_name("-flag").is_err());
        assert!(validate_profile_name("--help").is_err());
    }

    #[test]
    fn test_special_chars_blocked() {
        assert!(validate_profile_name("foo bar").is_err());
        assert!(validate_profile_name("foo;bar").is_err());
        assert!(validate_profile_name("$(whoami)").is_err());
    }
}
