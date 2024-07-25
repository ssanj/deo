use std::path::PathBuf;

use clap::Parser;

/// Automating handbrake to work with mkv-renamer.
///
/// Note: Your handbrake profiles should be created under ~/.deo/profiles after exporting from the handbrake UI.
#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct Args {

   /// Source directory that contains Rips/sessionX/renames and Encodes
   #[arg(short, long)]
   pub source: PathBuf,

   /// Verbose debug logging
   ///
   /// You can get very detailed logging of what deo is considering when using verbose logging
   #[arg(long)]
   pub verbose: bool
}

pub fn get_cli_args() -> Args {
  Args::parse()
}
