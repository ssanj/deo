use crate::args::cli::Args;
use crate::{file_mapper, handbrake, user_choices};
use crate::profiles::{self, ProfileConfig};
use crate::colours;
use console::style;


pub fn perform(args: Args) {
  match profiles::read_profile_config() {
    Ok(profile_config) => encode_profiles(args, profile_config),
    Err(error) => eprintln!("{}", style(error).bg(colours::RED))
  }
}


fn encode_profiles(args: Args, profile_config: ProfileConfig) {
  let sessions_to_encode_dir = file_mapper::get_session_encode_mapping(args.source, args.verbose);
  if sessions_to_encode_dir.is_empty() {
    println!("Could not find any renames to encode")
  } else {
    match user_choices::interact_with_user(sessions_to_encode_dir, profile_config) {
      user_choices::Interaction::ProceedToEncode(selections) => {
        match handbrake::encoder_with_handbrake(selections) {
          Ok(_) => (),
          Err(error) => eprintln!("{}", style(error).bg(colours::RED)),
        }
      },
      user_choices::Interaction::NoFilesToEncode => eprintln!("{}", style("No files to encode").bg(colours::RED)),
      user_choices::Interaction::CancelEncode => println!("User cancelled encoding"),
      user_choices::Interaction::InteractionError(error) => eprintln!("{}", style(format!("Interaction with the user raised an error: {}", error)).bg(colours::RED)),
    }
  }
}
