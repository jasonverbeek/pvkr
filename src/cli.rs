use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use owo_colors::OwoColorize;

#[derive(Debug, Parser)]
#[command(name = "pvkr")]
#[command(about = "A CLI tool for splicing and welding files for transfer")]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Debug, Subcommand)]
pub enum CliCommand {
    Split(SplitArgs),
    Weld(WeldArgs),
}

#[derive(Debug, Args)]
pub struct SplitArgs {
    #[arg(short, long, help = "The file to split")]
    pub file: PathBuf,
    #[arg(
        short,
        long,
        help = "The directory to output the split files in (does not need to exist)"
    )]
    pub output: PathBuf, // required because no sensible default
    #[arg(long, help = "Overwrite the directory if it is not empty")]
    pub overwrite: bool,
    #[arg(
        short = 's',
        long = "size",
        help = "The size of each chunk in bytes",
        default_value_t = 100
    )]
    pub chunk_size_bytes: u128,
}

#[derive(Debug, Args)]
pub struct WeldArgs {
    #[arg(
        short = 'd',
        long = "directory",
        help = "The directory to weld, containing a package.pvkr file"
    )]
    pub target_dir: PathBuf,
    #[arg(short, long, help = "The file to output to")]
    pub output: PathBuf,
    #[arg(long, help = "Overwrite the output file if it exists")]
    pub overwrite: bool,
}

fn term_out<S1: ToString, S2: AsRef<str>>(status: S1, msg: S2) {
    let now = chrono::Local::now();
    println!(
        "[{}][{}]: {}",
        now.format("%H:%M:%S"),
        status.to_string(),
        msg.as_ref()
    );
}

pub fn term_ok<S: AsRef<str>>(msg: S) {
    term_out("OK ".green(), msg.as_ref());
}

pub fn term_warn<S: AsRef<str>>(msg: S) {
    term_out("WRN".yellow(), msg.as_ref());
}

pub fn term_err<S: AsRef<str>>(msg: S) {
    term_out("ERR".red(), msg.as_ref());
}
