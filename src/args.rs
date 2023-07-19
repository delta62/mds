use clap::{Args as ClapArgs, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,

    pub mds_file: PathBuf,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Convert(ConvertArgs),
    Info,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum OutputFormat {
    Iso,
    Cue,
}

#[derive(ClapArgs, Debug)]
pub struct ConvertArgs {
    #[arg(long, value_enum)]
    pub format: OutputFormat,
}
