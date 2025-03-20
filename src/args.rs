use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command()]
/// This is a simple utility to manage configs / themes / icons / font / and more
pub struct AppArgs {
    /// Your lua config file
    #[arg(short, long)]
    pub config: PathBuf,

    /// Only update, this ignors force in lua config
    #[arg(short,long, action)]
    pub update: bool,
}
