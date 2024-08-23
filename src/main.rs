mod cli;
mod error;
mod sha256;
mod split;
mod util;
mod weld;

use clap::Parser;

use crate::cli::{term_err, term_ok, Cli, CliCommand};

fn main() {
    // TODO: look into multi-threading, but this is a whole mess, how many cpus? are there < chunks than cpus? etc
    let args = Cli::parse();

    let exit_status = match args.command {
        CliCommand::Split(args) => crate::split::split(args),
        CliCommand::Weld(args) => crate::weld::weld(args),
    };

    match exit_status {
        Ok(_) => term_ok("Operation completed successfully"),
        Err(e) => {
            term_err(format!("Operation failed: {}", e));
            std::process::exit(1);
        }
    }
}
