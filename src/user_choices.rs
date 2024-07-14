use console::style;

use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use crate::profiles::ProfileConfig;
use crate::user_selection::{ContinueType, UserSelection};
use crate::entry_type::SessionToEncodeDir;

#[allow(clippy::enum_variant_names)]
pub enum Interaction {
  ProceedToEncode(Vec<UserSelection>),
  NoFilesToEncode,
  CancelEncode,
  InteractionError(String),
}

pub fn interact_with_user(sessions_to_encode_dir: Vec<SessionToEncodeDir>, profiles: ProfileConfig) -> Interaction {
    let selections = get_user_selection(sessions_to_encode_dir, profiles);
    println!("Your choices were:");

    if selections.is_empty() {
      println!("You made no choices");
      Interaction::NoFilesToEncode
    } else {
      for selection in &selections {
        println!("  {}", selection);
      }

      let continue_options =
        [
          ContinueType::EncodeSelection,
          ContinueType::Cancel
        ];

      let continue_result = show_select(&continue_options, "Proceed with encoding selection?");
      match continue_result {
        Ok(ContinueType::EncodeSelection) => {
          Interaction::ProceedToEncode(selections)
        },
        Ok(ContinueType::Cancel) => Interaction::CancelEncode,
        Err(e) => Interaction::InteractionError(e.to_string())
      }
    }
}


fn get_user_selection(sessions_to_encode_dir: Vec<SessionToEncodeDir>, profiles: ProfileConfig) -> Vec<UserSelection> {
  let profile_options = profiles.items();

  let mut selections: Vec<UserSelection> = vec![];

  println!();
  for sed in sessions_to_encode_dir {
    let files = sed.session().files();
    let num = files.iter().count();
    println!("{} ({}) has the following {} files:", style(&sed.encode_dir().season).underlined(),  style(sed.session_id().id()).yellow().bold(), num);
    for file in files {
      println!(" - {}", file.mkv_file);
    }

    let selected_profile = show_select(profile_options, "Select encoding profile:").unwrap();
    selections.push(UserSelection::new(sed.session_id().clone(), sed, selected_profile.clone()));
    println!()
  }

  selections
}

fn show_select<'a, T: ToString>(options: &'a [T], prompt: &str) -> Result<&'a T, String> {
    FuzzySelect::with_theme(&ColorfulTheme::default())
      .with_prompt(prompt)
      .default(0)
      .items(options)
      .interact()
      .map_err(|e| e.to_string())
      .and_then(|index| {
        options
          .get(index)
          .ok_or_else(|| "Invalid selection index".to_owned())
      })
}
