use clap::Parser;
use crate::args::Args;
use furbrowser_rs::interactive;

mod args;

fn main() {
    let args = Args::parse();

    if let Err(e) = interactive(&args.profile, args.append) {
        eprintln!("error: {e}")
    }
}
