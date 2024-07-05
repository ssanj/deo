use std::collections::HashMap;
use walkdir::WalkDir;
use regex::Regex;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

use args::cli;
use entry_type::{EntryType, EncodeType, SessionType, SessionId};

mod args;
mod entry_type;


fn main() {
  let args = cli::get_cli_args();
  println!("source: {}", &args.source.to_string_lossy());
  let rename_reg = Regex::new(r"(session\d{1,})\/renames\/(S\d{2,}E\d{2,})\s-\s(.+.mkv)$").unwrap();
  let encode_reg = Regex::new(r".+\/(.+\s-\sSeason\s\d{2,})$").unwrap();

  let sessions: Vec<EntryType> =
    WalkDir::new(&args.source)
      .into_iter()
      .filter_map(|de| de.ok())
      .filter_map(|de| {
        if de.file_type().is_file() && rename_reg.is_match(de.path().to_str().unwrap()){
          if let Some((_, [session, episode, file])) = rename_reg.captures(de.path().to_str().unwrap()).map(|c| c.extract()) {
            Some(EntryType::new_session(de.path(), session, episode, file))
          } else {
            None
          }
        } else if de.file_type().is_dir() && encode_reg.is_match(de.path().to_str().unwrap()) {
          if let Some((_, [season])) = encode_reg.captures(de.path().to_str().unwrap()).map(|c| c.extract()) {
            Some(EntryType::new_encodes(de.path(), season))
          } else {
            None
          }
        } else {
          None
        }
      })
      .collect();

  let session_names: Vec<String> =
    sessions
      .iter()
      .map(|s| format!("{:?}", s))
      .collect();

  let mut session_hash: HashMap<SessionId, Vec<SessionType>> = HashMap::new();

  for session in sessions.iter() {
    if let Ok(session_type) = <EntryType as TryInto<SessionType>>::try_into(session.clone()) {
      let session_id = SessionId::new(&session_type.session);
      session_hash
        .entry(session_id)
        .and_modify(|sv| sv.push(session_type.clone()))
        .or_insert(vec!(session_type));
    }
  }

  let encode_values: Vec<_> =
    sessions
      .into_iter()
      .filter_map(|s| <EntryType as TryInto<EncodeType>>::try_into(s).ok() )
      .collect();

  let mut options: Vec<String> =
    encode_values
      .iter()
      .map(|v| format!("{}", v.season))
      .collect();

  options.push("Skip".to_owned());
  options.push("Done".to_owned());

  println!();
  for (session_id, session_files) in session_hash {
    println!("session_id: {}", session_id.id());
    for file in  session_files {
      println!(" - {}", file.file);
    }

    let selection =
      show_select(&options, &format!("Copy {} to: ", session_id.id())).unwrap();
    println!()
  }
}

fn show_select(options: &[String], prompt: &str) -> Result<String, String> {
    FuzzySelect::with_theme(&ColorfulTheme::default())
      .with_prompt(prompt)
      .default(0)
      .items(&options)
      .interact()
      .map_err(|e| e.to_string())
      .and_then(|index| {
          options
            .get(index)
            .ok_or_else(|| "Invalid selection index".to_owned())
            .map(|v| v.to_owned())
      })
}
