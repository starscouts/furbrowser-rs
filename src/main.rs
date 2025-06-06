use clap::Parser;
use colored::Colorize;
use furbrowser_rs::start_tui;

use crate::args::Args;

mod args;

fn main() {
    let args = Args::parse();

    if let Err(e) = start_tui(&args.profile, args.append) {
        eprintln!("{} {e}", "error:".red().bold())
    }
}
