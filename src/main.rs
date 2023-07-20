mod args;
mod convert;
mod error;
mod info;
mod loader;
mod mds;
mod timecode;
mod util;

use args::{Args, Command, ConvertArgs, OutputFormat};
use clap::Parser;
use convert::{convert_to_cue_bin, convert_to_iso};
use info::info;

fn main() {
    let args = Args::parse();

    let result = match &args.command {
        Command::Info(args) => info(&args.mds_file),
        Command::Convert(ConvertArgs { mds_file, format }) => match format {
            OutputFormat::Iso => convert_to_iso(&mds_file),
            OutputFormat::Cue => convert_to_cue_bin(&mds_file),
        },
    };

    if let Err(err) = result {
        eprintln!("{err}");
    }
}
