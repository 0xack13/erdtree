use std::process::ExitCode;

use clap::{CommandFactory, FromArgMatches};
use render::{
    context::Context,
    tree::{self, Tree},
};

/// Filesystem operations.
mod fs;

/// Dev icons.
mod icons;

/// Tools and operations to display root-directory interact with ANSI configs.
mod render;

/// Common utilities.
mod utils;

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    tree::ui::init();
    let matches = Context::command().args_override_self(true).get_matches();
    let ctx = Context::from_arg_matches(&matches)?;

    let tree = Tree::init(ctx)?;

    println!("{tree}");

    Ok(())
}
