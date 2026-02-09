use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};

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

/// Strips a trailing `.gpg` extension so users can pass either `name` or `name.gpg`.
fn normalize_profile_name(name: &str) -> &str {
    name.strip_suffix(".gpg").unwrap_or(name)
}

/// Resolves the profile path, preferring the encrypted `.gpg` variant if it exists.
pub fn resolve_profile_path(profile_name: &str) -> Result<PathBuf> {
    let (plain, gpg) = profile_paths(profile_name)?;
    Ok(if gpg.exists() { gpg } else { plain })
}

/// Returns both the plain and encrypted paths for a profile, validating the name.
pub fn profile_paths(profile_name: &str) -> Result<(PathBuf, PathBuf)> {
    let name = normalize_profile_name(profile_name);
    validate_profile_name(name)?;
    let profile_dir = get_profile_dir()?;
    let plain = profile_dir.join(name);
    let gpg = profile_dir.join(format!("{name}.gpg"));
    Ok((plain, gpg))
}

/// Returns `true` if the path has a `.gpg` extension.
pub fn is_encrypted(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == "gpg")
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

    #[test]
    fn test_is_encrypted() {
        use std::path::Path;
        assert!(is_encrypted(Path::new("/some/path/profile.gpg")));
        assert!(!is_encrypted(Path::new("/some/path/profile")));
        assert!(!is_encrypted(Path::new("/some/path/profile.txt")));
        assert!(!is_encrypted(Path::new("profile.gpg.bak")));
    }

    #[test]
    fn test_resolve_profile_path_validates_name() {
        assert!(resolve_profile_path("").is_err());
        assert!(resolve_profile_path("..").is_err());
        assert!(resolve_profile_path("foo/bar").is_err());
    }

    #[test]
    fn test_profile_paths_validates_name() {
        assert!(profile_paths("").is_err());
        assert!(profile_paths("..").is_err());
        assert!(profile_paths("foo/bar").is_err());
    }

    #[test]
    fn test_profile_paths_returns_plain_and_gpg() {
        let (plain, gpg) = profile_paths("test-profile").unwrap();
        assert!(plain.ends_with("test-profile"));
        assert!(gpg.ends_with("test-profile.gpg"));
        assert_eq!(plain.parent(), gpg.parent());
    }

    #[test]
    fn test_resolve_profile_path_returns_plain_for_valid_name() {
        let path = resolve_profile_path("some-profile").unwrap();
        assert!(path.ends_with("some-profile"));
        assert!(!path.to_string_lossy().ends_with(".gpg"));
    }

    #[test]
    fn test_normalize_strips_gpg_suffix() {
        assert_eq!(normalize_profile_name("my-profile.gpg"), "my-profile");
        assert_eq!(normalize_profile_name("my-profile"), "my-profile");
        assert_eq!(normalize_profile_name("a.b.gpg"), "a.b");
    }

    #[test]
    fn test_profile_paths_normalizes_gpg_suffix() {
        let (plain_a, gpg_a) = profile_paths("test-profile").unwrap();
        let (plain_b, gpg_b) = profile_paths("test-profile.gpg").unwrap();
        assert_eq!(plain_a, plain_b);
        assert_eq!(gpg_a, gpg_b);
    }
}
