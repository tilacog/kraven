# kraven

Environment profile manager for named environment variable profiles.

## Installation

```bash
cargo install kraven
```

## Usage

```bash
# List available profiles
kraven list

# Create or edit a profile
kraven edit my-profile

# Activate a profile (spawns a subshell with env vars)
kraven activate my-profile

# Show the currently active profile
kraven current

# Display profile contents
kraven show my-profile

# Display profile contents with masked values
kraven show my-profile --mask

# Show how to exit the current kraven session
kraven deactivate

# Remove a profile
kraven remove my-profile

# Show shell completion setup instructions
kraven completions
```

## Profile Format

Profiles are stored as plain text files in `~/.config/kraven/` using the standard dotenv format:

```
KEY=value
ANOTHER_KEY=another_value

# Comments start with #
QUOTED="value with spaces"
SINGLE_QUOTED='literal $value without expansion'

# Double-quoted values support escape sequences
ESCAPED="line1\nline2\ttabbed"
```

Supported escape sequences in double-quoted values: `\"`, `\\`, `\n`, `\t`

## License

GPL-3.0
