//! Shell completion support using clap's dynamic completion system.

use clap::CommandFactory;
use clap_complete::engine::{ArgValueCompleter, CompletionCandidate};
use clap_complete::CompleteEnv;
use std::ffi::OsStr;

use crate::config;
use crate::Cli;

/// Initialize dynamic shell completions.
///
/// This must be called before argument parsing. If the `COMPLETE` environment
/// variable is set, it handles the completion request and exits.
pub fn init() {
    CompleteEnv::with_factory(build_cli).complete();
}

/// Build the CLI command with profile completers attached.
fn build_cli() -> clap::Command {
    Cli::command()
        .mut_subcommand("activate", add_profile_completer)
        .mut_subcommand("edit", add_profile_completer)
        .mut_subcommand("show", add_profile_completer)
        .mut_subcommand("remove", add_profile_completer)
}

/// Add profile completer to a subcommand's "profile" argument.
fn add_profile_completer(cmd: clap::Command) -> clap::Command {
    cmd.mut_arg("profile", |arg| {
        arg.add(ArgValueCompleter::new(complete_profiles))
    })
}

/// Complete profile names from the profile directory.
fn complete_profiles(current: &OsStr) -> Vec<CompletionCandidate> {
    let current_str = current.to_string_lossy();

    let Ok(profile_dir) = config::get_profile_dir() else {
        return vec![];
    };

    let Ok(entries) = std::fs::read_dir(&profile_dir) else {
        return vec![];
    };

    entries
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            if !path.is_file() {
                return None;
            }
            let name = path.file_name()?.to_str()?;
            if name.starts_with('.') {
                return None;
            }
            if name.starts_with(&*current_str) {
                Some(CompletionCandidate::new(name))
            } else {
                None
            }
        })
        .collect()
}
