mod args;
mod convert;
mod error;
mod info;
mod mds;

use args::{Args, Command};
use clap::Parser;
use convert::convert;
use info::info;

fn main() {
    let args = Args::parse();

    let result = match args.command {
        Command::Info => info(&args),
        Command::Convert => convert(&args),
    };

    if let Err(err) = result {
        eprintln!("{err}");
    }
}
