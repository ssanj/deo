use args::cli::{self, Args};
use error::DeoProfileError;
use profiles::ProfileConfig;

mod args;
mod entry_type;
mod user_selection;
mod hb_output_parser;
mod file_mapper;
mod user_choices;
mod handbrake;
mod profiles;
mod error;
mod debug;
mod colours;

fn main() {
  let args = cli::get_cli_args();
  match profiles::read_profile_config() {
    Ok(profile_config) => encode_profiles(args, profile_config),
    Err(error) => eprintln!("{}", error)
  }
}

fn encode_profiles(args: Args, profile_config: ProfileConfig) {
  let sessions_to_encode_dir = file_mapper::get_session_encode_mapping(args.source, args.verbose);
  if sessions_to_encode_dir.is_empty() {
    println!("Could not find any renames to encode")
  } else {
    match user_choices::interact_with_user(sessions_to_encode_dir, profile_config) {
      user_choices::Interaction::ProceedToEncode(selections) => {
        match handbrake::encode(selections) {
          Ok(_) => (),
          Err(error) => eprintln!("{}", error),
        }
      },
      user_choices::Interaction::NoFilesToEncode => eprintln!("No files to encode"),
      user_choices::Interaction::CancelEncode => println!("User canceled encoding"),
      user_choices::Interaction::InteractionError(error) => eprintln!("Interaction with the user raised an error: {}", error),
    }
  }
}
