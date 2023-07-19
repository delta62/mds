mod args;
mod convert;
mod error;
mod info;
mod mds;

use args::{Args, Command, ConvertArgs, OutputFormat};
use clap::Parser;
use convert::{convert_to_cue_bin, convert_to_iso};
use info::info;

fn main() {
    let args = Args::parse();

    let result = match &args.command {
        Command::Info => info(&args),
        Command::Convert(ConvertArgs { format }) => match format {
            OutputFormat::Iso => convert_to_iso(&args),
            OutputFormat::Cue => convert_to_cue_bin(&args),
        },
    };

    if let Err(err) = result {
        eprintln!("{err}");
    }
}
