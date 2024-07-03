use std::path::PathBuf;

use clap::Parser;

/// automating handbrake to work with mkv-renamer
#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct Args {

   /// Source directory that contains Rips/sessionX/renames and Encodes
   #[arg(short, long)]
   pub source: PathBuf,

   /// Verbose debug logging
   #[arg(long)]
   pub verbose: bool
}

pub fn get_cli_args() -> Args {
  Args::parse()
}
