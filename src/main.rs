use args::cli;

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

fn main() {
  let args = cli::get_cli_args();
  let profiles = profiles::read_profile_config().unwrap();

  let sessions_to_encode_dir = file_mapper::get_session_encode_mapping(args.source, args.verbose);
  if sessions_to_encode_dir.is_empty() {
    println!("Could not find any renames to encode")
  } else {
    match user_choices::interact_with_user(sessions_to_encode_dir, profiles) {
      user_choices::Interaction::ProceedToEncode(selections) => {
        match handbrake::encode(selections) {
          Ok(_) => (),
          Err(error) => println!("{}", error),
        }
      },
      user_choices::Interaction::NoFilesToEncode => println!("No files to encode"),
      user_choices::Interaction::CancelEncode => println!("User canceled encoding"),
      user_choices::Interaction::InteractionError(error) => println!("Interaction with the user raised an error: {}", error),
    }
  }
}
