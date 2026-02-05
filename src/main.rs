//! kraven - Environment profile manager for named environment variable profiles.

use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_complete::Shell;

mod commands;
mod config;
mod profile;

/// CLI for managing environment variable profiles.
#[derive(Parser)]
#[command(name = "kraven")]
#[command(author, version, about = "Environment profile manager")]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Activate a profile (spawn a subshell with the profile's env vars)
    Activate {
        /// Name of the profile to activate
        profile: String,
    },

    /// Show how to exit the current kraven session
    Deactivate,

    /// List available profiles
    #[command(visible_alias = "ls")]
    List,

    /// Create or edit a profile using $EDITOR
    Edit {
        /// Name of the profile to edit
        profile: String,
    },

    /// Display profile contents
    Show {
        /// Name of the profile to show
        profile: String,

        /// Mask sensitive values
        #[arg(short, long)]
        mask: bool,
    },

    /// Remove a profile
    #[command(visible_alias = "rm")]
    Remove {
        /// Name of the profile to remove
        profile: String,

        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
    },

    /// Show the currently active profile
    Current,

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },

    /// List profile names for shell completion (hidden)
    #[command(hide = true)]
    CompleteProfiles,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Activate { profile } => commands::activate::run(&profile),
        Commands::Deactivate => commands::deactivate::run(),
        Commands::List => commands::list::run(),
        Commands::Edit { profile } => commands::edit::run(&profile),
        Commands::Show { profile, mask } => commands::show::run(&profile, mask),
        Commands::Remove { profile, force } => commands::remove::run(&profile, force),
        Commands::Current => commands::current::run(),
        Commands::Completions { shell } => commands::completions::run(shell),
        Commands::CompleteProfiles => commands::completions::list_profiles(),
    }
}
