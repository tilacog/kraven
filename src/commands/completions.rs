use anyhow::Result;

/// Print shell completion setup instructions for all supported shells.
#[allow(clippy::unnecessary_wraps)]
pub fn run() -> Result<()> {
    println!("Add one of the following lines to your shell configuration:\n");

    println!("Bash (~/.bashrc):");
    println!("  source <(COMPLETE=bash kraven)\n");

    println!("Elvish (~/.elvish/rc.elv):");
    println!("  eval (E:COMPLETE=elvish kraven | slurp)\n");

    println!("Fish (~/.config/fish/config.fish):");
    println!("  COMPLETE=fish kraven | source\n");

    println!("PowerShell ($PROFILE):");
    println!(
        "  $env:COMPLETE = \"powershell\"; kraven | Out-String | Invoke-Expression; \
         Remove-Item Env:\\COMPLETE\n"
    );

    println!("Zsh (~/.zshrc):");
    println!("  source <(COMPLETE=zsh kraven)\n");

    println!("Then restart your shell or source the config file.");

    Ok(())
}
