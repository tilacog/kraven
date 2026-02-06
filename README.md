# kraven

Manage named environment variable profiles.

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

## Shell Completions

Enable tab completion by adding the appropriate line to your shell config:

```bash
# Bash (~/.bashrc)
source <(COMPLETE=bash kraven)

# Zsh (~/.zshrc)
source <(COMPLETE=zsh kraven)

# Fish (~/.config/fish/config.fish)
COMPLETE=fish kraven | source
```

Then restart your shell or source the config file.

## Customizing Your Shell Prompt

When a profile is active, Kraven sets the `KRAVEN_ACTIVE` environment variable to the profile name. You can use this to display the active profile in your shell prompt.

### Zsh

Add this to your `~/.zshrc`:

```zsh
precmd() {
    if [[ -n "$KRAVEN_ACTIVE" ]]; then
        kraven_info="[${KRAVEN_ACTIVE}] "
    else
        kraven_info=""
    fi
}

setopt PROMPT_SUBST
PROMPT='${kraven_info}%~ %# '
```

### Bash

Add this to your `~/.bashrc`:

```bash
set_prompt() {
    if [[ -n "$KRAVEN_ACTIVE" ]]; then
        kraven_info="[${KRAVEN_ACTIVE}] "
    else
        kraven_info=""
    fi
    PS1="${kraven_info}\w \$ "
}
PROMPT_COMMAND=set_prompt
```

### Fish

Add this to your `~/.config/fish/config.fish` or create `~/.config/fish/functions/fish_prompt.fish`:

```fish
function fish_prompt
    if set -q KRAVEN_ACTIVE
        echo -n "[$KRAVEN_ACTIVE] "
    end
    echo -n (prompt_pwd) '> '
end
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
