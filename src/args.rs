use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(value_enum)]
    pub command: Command,

    pub mds_file: PathBuf,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Command {
    Convert,
    Info,
}
