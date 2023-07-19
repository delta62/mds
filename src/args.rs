use clap::{Args as ClapArgs, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// Utilities for reading and converting .mds/.mdf disk image files
#[derive(Debug, Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Convert .mdf/.mds files to other formats
    Convert(ConvertArgs),

    /// Print metadata contained by .mds files
    Info(InfoArgs),
}

#[derive(Clone, Debug, ValueEnum)]
pub enum OutputFormat {
    /// Convert into a .iso file. ISO files can only contain one track.
    Iso,
    /// Convert into .bin and .cue files. This format supports multiple tracks.
    Cue,
}

#[derive(ClapArgs, Debug)]
pub struct InfoArgs {
    /// Path to the .mds file to print information about
    pub mds_file: PathBuf,
}

#[derive(ClapArgs, Debug)]
pub struct ConvertArgs {
    /// Path to the .mds file to convert
    pub mds_file: PathBuf,

    /// The format to convert into
    #[arg(long, value_enum)]
    pub format: OutputFormat,
}
