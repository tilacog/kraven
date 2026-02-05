use anyhow::{bail, Context, Result};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Profile {
    pub vars: BTreeMap<String, String>,
}

impl Profile {
    /// Parse profile content from a string.
    pub fn parse(content: &str, path: &Path) -> Result<Self> {
        let vars = parse_env_file(content, path)?;
        Ok(Self { vars })
    }

    /// Load a profile from the given path.
    pub fn load(name: &str, path: &Path) -> Result<Self> {
        let content =
            fs::read_to_string(path).with_context(|| format!("Failed to read profile '{name}'"))?;
        Self::parse(&content, path)
    }
}

fn parse_env_file(content: &str, path: &Path) -> Result<BTreeMap<String, String>> {
    let mut vars = BTreeMap::new();

    for (line_num, line) in content.lines().enumerate() {
        let line_num = line_num + 1; // 1-indexed for error messages
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Find the first '=' to split key and value
        let Some(eq_pos) = line.find('=') else {
            bail!(
                "{}:{line_num}: Invalid line (missing '='): {line}",
                path.display(),
            );
        };

        let (key, value) = line.split_at(eq_pos);
        let key = key.trim();
        let value = value[1..].trim(); // Skip the '='

        // Validate key
        if key.is_empty() {
            bail!("{}:{line_num}: Empty variable name: {line}", path.display(),);
        }

        if !is_valid_env_name(key) {
            bail!(
                "{}:{line_num}: Invalid variable name '{key}': must contain only alphanumeric characters and underscores, and not start with a digit",
                path.display(),
            );
        }

        // Parse value (handle quotes)
        let parsed_value = parse_value(value, path, line_num)?;

        vars.insert(key.to_string(), parsed_value);
    }

    Ok(vars)
}

fn is_valid_env_name(name: &str) -> bool {
    let mut chars = name.chars();

    // First character must be letter or underscore
    let is_valid_first = chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_');

    // Rest must be alphanumeric or underscore
    is_valid_first && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn parse_value(value: &str, path: &Path, line_num: usize) -> Result<String> {
    if value.is_empty() {
        return Ok(String::new());
    }

    let first_char = value.chars().next().unwrap();
    let last_char = value.chars().next_back().unwrap();

    // Check for double-quoted strings
    if first_char == '"' && last_char == '"' && value.len() >= 2 {
        let inner = &value[1..value.len() - 1];
        return Ok(unescape_double_quoted(inner));
    }

    // Check for single-quoted strings (no escape processing)
    if first_char == '\'' && last_char == '\'' && value.len() >= 2 {
        return Ok(value[1..value.len() - 1].to_string());
    }

    // Check for unclosed quotes
    if first_char == '"' || first_char == '\'' {
        bail!(
            "{}:{line_num}: Unclosed quote in value: {value}",
            path.display(),
        );
    }

    // Unquoted value
    Ok(value.to_string())
}

fn unescape_double_quoted(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.peek() {
                Some('"') => {
                    result.push('"');
                    chars.next();
                }
                Some('\\') => {
                    result.push('\\');
                    chars.next();
                }
                Some('n') => {
                    result.push('\n');
                    chars.next();
                }
                Some('t') => {
                    result.push('\t');
                    chars.next();
                }
                _ => {
                    // Unknown escape, keep as-is
                    result.push(c);
                }
            }
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_path() -> PathBuf {
        PathBuf::from("test.env")
    }

    #[test]
    fn test_parse_simple_values() {
        let content = "KEY=value\nANOTHER=123";
        let vars = parse_env_file(content, &test_path()).unwrap();
        assert_eq!(vars.get("KEY"), Some(&"value".to_string()));
        assert_eq!(vars.get("ANOTHER"), Some(&"123".to_string()));
    }

    #[test]
    fn test_parse_comments_and_empty_lines() {
        let content = "# This is a comment\nKEY=value\n\n# Another comment\nKEY2=value2";
        let vars = parse_env_file(content, &test_path()).unwrap();
        assert_eq!(vars.len(), 2);
        assert_eq!(vars.get("KEY"), Some(&"value".to_string()));
        assert_eq!(vars.get("KEY2"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_parse_double_quoted_value() {
        let content = "KEY=\"value with spaces\"";
        let vars = parse_env_file(content, &test_path()).unwrap();
        assert_eq!(vars.get("KEY"), Some(&"value with spaces".to_string()));
    }

    #[test]
    fn test_parse_single_quoted_value() {
        let content = "KEY='literal $value'";
        let vars = parse_env_file(content, &test_path()).unwrap();
        assert_eq!(vars.get("KEY"), Some(&"literal $value".to_string()));
    }

    #[test]
    fn test_parse_empty_value() {
        let content = "EMPTY=";
        let vars = parse_env_file(content, &test_path()).unwrap();
        assert_eq!(vars.get("EMPTY"), Some(&String::new()));
    }

    #[test]
    fn test_invalid_line_missing_equals() {
        let content = "INVALID_LINE";
        let result = parse_env_file(content, &test_path());
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_var_name_starts_with_digit() {
        let content = "1INVALID=value";
        let result = parse_env_file(content, &test_path());
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_var_name_with_underscore() {
        let content = "_VALID=value\nALSO_VALID=value2";
        let vars = parse_env_file(content, &test_path()).unwrap();
        assert_eq!(vars.get("_VALID"), Some(&"value".to_string()));
        assert_eq!(vars.get("ALSO_VALID"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_unclosed_quote() {
        let content = "KEY=\"unclosed";
        let result = parse_env_file(content, &test_path());
        assert!(result.is_err());
    }

    #[test]
    fn test_escaped_quotes_in_double_quoted() {
        let content = r#"KEY="value with \" escaped quote""#;
        let vars = parse_env_file(content, &test_path()).unwrap();
        assert_eq!(
            vars.get("KEY"),
            Some(&"value with \" escaped quote".to_string())
        );
    }

    #[test]
    fn test_escape_sequences() {
        let content = r#"KEY="line1\nline2\ttabbed\\backslash""#;
        let vars = parse_env_file(content, &test_path()).unwrap();
        assert_eq!(
            vars.get("KEY"),
            Some(&"line1\nline2\ttabbed\\backslash".to_string())
        );
    }

    #[test]
    fn test_single_quotes_no_escape() {
        // Single quotes should not process escapes
        let content = r"KEY='literal \n not newline'";
        let vars = parse_env_file(content, &test_path()).unwrap();
        assert_eq!(
            vars.get("KEY"),
            Some(&r"literal \n not newline".to_string())
        );
    }

    #[test]
    fn test_utf8_in_values() {
        let content = "KEY=café\nKEY2=\"日本語\"";
        let vars = parse_env_file(content, &test_path()).unwrap();
        assert_eq!(vars.get("KEY"), Some(&"café".to_string()));
        assert_eq!(vars.get("KEY2"), Some(&"日本語".to_string()));
    }
}
