use anyhow::Result;

use crate::config::get_profile_path;
use crate::profile::Profile;

pub fn run(profile_name: &str, mask_values: bool) -> Result<()> {
    let profile_path = get_profile_path(profile_name)?;
    let profile = Profile::load(profile_name, &profile_path)?;

    if profile.vars.is_empty() {
        println!("Profile '{profile_name}' is empty.");
        return Ok(());
    }

    // BTreeMap maintains sorted order, so no explicit sorting needed
    for (key, value) in &profile.vars {
        if mask_values {
            println!("{key}={}", mask_value(value));
        } else if value.contains(' ') || value.contains('"') || value.contains('\'') {
            let escaped = value.replace('"', "\\\"");
            println!("{key}=\"{escaped}\"");
        } else {
            println!("{key}={value}");
        }
    }

    Ok(())
}

fn mask_value(value: &str) -> String {
    let char_count = value.chars().count();

    if char_count <= 4 {
        return "*".repeat(char_count);
    }

    // Show first 2 and last 2 characters
    let start: String = value.chars().take(2).collect();
    let end: String = value.chars().skip(char_count - 2).collect();
    let middle = "*".repeat(char_count - 4);
    format!("{start}{middle}{end}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_empty() {
        assert_eq!(mask_value(""), "");
    }

    #[test]
    fn test_mask_short() {
        assert_eq!(mask_value("ab"), "**");
        assert_eq!(mask_value("abcd"), "****");
    }

    #[test]
    fn test_mask_long() {
        assert_eq!(mask_value("abcdef"), "ab**ef");
        assert_eq!(mask_value("secretkey123"), "se********23");
    }

    #[test]
    fn test_mask_unicode() {
        // This would panic with byte indexing
        assert_eq!(mask_value("cafén"), "ca*én");
        // "🔑secret🔒" is 8 chars: 🔑 s e c r e t 🔒
        // First 2: 🔑 s, Last 2: t 🔒, Middle 4: ****
        assert_eq!(mask_value("🔑secret🔒"), "🔑s****t🔒");
    }
}
