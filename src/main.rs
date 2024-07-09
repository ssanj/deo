use std::collections::HashMap;
use std::path::Path;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use walkdir::WalkDir;
use regex::Regex;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use std::process::{Command, Stdio};

use args::cli;
use entry_type::{EntryType, EncodeDir, RenameFile, SessionId, Session, SessionToEncodeDir};

use crate::user_selection::{ContinueType, EncodeOption, Profile, UserSelection};
use crate::hb_output_parser::{parse, Output};

mod args;
mod entry_type;
mod user_selection;
mod hb_output_parser;

fn main() {
  let args = cli::get_cli_args();
  println!("source: {}", &args.source.to_string_lossy());
  let rename_file_reg = Regex::new(r"(session\d{1,})\/renames\/(S\d{2,}E\d{2,})\s-\s(.+.mkv)$").unwrap();
  let encode_file_reg = Regex::new(r"(session\d{1,})\/renames\/encode\.txt$").unwrap();
  let encode_dir_reg = Regex::new(r".+\/(.+\s-\sSeason\s\d{2,})$").unwrap();

  let entry_types: Vec<EntryType> =
    WalkDir::new(&args.source)
      .into_iter()
      .filter_map(|de| de.ok())
      .filter_map(|de| {
        if de.file_type().is_file() && rename_file_reg.is_match(de.path().to_str().unwrap()){
          if let Some((_, [session, episode, file])) = rename_file_reg.captures(de.path().to_str().unwrap()).map(|c| c.extract()) {
            Some(EntryType::new_rename(de.path(), session, episode, file))
          } else {
            None
          }
        } /*else if de.file_type().is_dir() && encode_dir_reg.is_match(de.path().to_str().unwrap()) {
          if let Some((_, [season])) = encode_dir_reg.captures(de.path().to_str().unwrap()).map(|c| c.extract()) {
            Some(EntryType::new_encodes(de.path(), season))
          } else {
            None
          }
        } */
          else if de.file_type().is_file() && encode_file_reg.is_match(de.path().to_str().unwrap()) {
           if let Some((_, [session])) = encode_file_reg.captures(de.path().to_str().unwrap()).map(|c| c.extract()) {
            std::fs::read_to_string(de.path())
              .ok()
              .map(|encode_file_contents| encode_file_contents.trim().to_owned()) // remove newline added by read_to_string
              .and_then(|encode_file_contents| {
                let encode_dir = Path::new(&encode_file_contents);
                if encode_dir.is_dir() && encode_dir_reg.is_match(&encode_file_contents) {
                  if let Some((_, [season])) = encode_dir_reg.captures(&encode_file_contents).map(|c| c.extract()) {
                    Some(EntryType::new_encodes(&encode_file_contents, season, session))
                  } else {
                    None
                  }
                } else {
                  None
                }
              })
          } else {
            None
          }
        } else {
          None
        }
      })
      .collect();


  let sessions_hash: HashMap<SessionId, Session> =
    entry_types
      .iter()
      .filter_map(|session_type| {
        <EntryType as TryInto<RenameFile>>::try_into(session_type.clone())
          .ok()
        })
      .collect();


  println!("sessions_hash \n{:?}", sessions_hash);

  let encode_dir_hash: HashMap<SessionId, EncodeDir> =
    entry_types
      .into_iter()
      .filter_map(|s| {
        <EntryType as TryInto<EncodeDir>>::try_into(s)
          .ok()
          .map(|encode_dir| {
            (encode_dir.session_id.clone(), encode_dir)
          })
      })
      .collect();

  let mut sessions_to_encode_dir: Vec<SessionToEncodeDir> = vec![];

  // Map from SessionId -> SessionToEncodeDir
  for (session_id, session) in sessions_hash {
    if let Some(encode_dir) = encode_dir_hash.get(&session_id) {
      sessions_to_encode_dir.push(SessionToEncodeDir::new(session_id, session, encode_dir.clone()))
    }
  }

  // let mut encode_options: Vec<EncodeOption> =
  //   encode_values
  //     .into_iter()
  //     .map(EncodeOption::Encode)
  //     .collect();

  // encode_options.push(EncodeOption::Skip);
  // encode_options.push(EncodeOption::Done);

  let profile_options =
    vec![
      Profile::Dvd,
      Profile::Bluray
    ];

  let mut selections: Vec<UserSelection> = vec![];

  println!();
  for sed in sessions_to_encode_dir {
    println!("{} has the following files:", style(sed.session_id().id()).yellow().bold());
    for file in sed.session().files() {
      println!(" - {}", file.mkv_file);
    }

    let selected_profile = show_select(&profile_options, "Profile:").unwrap();
    selections.push(UserSelection::new(sed.session_id().clone(), sed, selected_profile.clone()))
  }

  println!("Your choices were:");

  if selections.is_empty() {
    println!("You made no choices")
  } else {
    for selection in &selections {
      println!("  {}", selection)
    }

    let continue_options =
      [
        ContinueType::EncodeSelection,
        ContinueType::Cancel
      ];

    let continue_result = show_select(&continue_options, "Proceed with encoding selection?");
    match continue_result {
      Ok(ContinueType::EncodeSelection) => {
        println!("{}", "you wanted to continue")
        // encode_selection(selections).unwrap()
      },
      Ok(ContinueType::Cancel) => println!("{}", "canceling"),
      Err(_) => println!("{}", "quitting.."),
    }
  }
}

fn encode_selection(selections: Vec<UserSelection>) -> Result<(), String> {
  println!("{}", "encoding...");
  //handbreakcli
  // --preset-import-file ~/Desktop/DVD\ -\ H265\ Apple\ Silicon\ HQ.json
  // -Z "DVD - H265 Apple Silicon HQ"
  // -i S05E01\ -\ Mr.\ Monk\ and\ the\ Actor.mkv
  // -o S05E01\ -\ Mr.\ Monk\ and\ the\ Actor.mp4

  // TODO: Source these from somewhere
  let profile_file = "/Users/sanj/Desktop/DVD - H265 Apple Silicon HQ.json";
  let profile_name = "DVD - H265 Apple Silicon HQ";

  let mut cmd = Command::new("handbrakecli");

  cmd
    .arg("--preset-import-file")
    .arg(profile_file)
    .arg("-Z")
    .arg(profile_name);


  let bar_style =
    ProgressStyle::with_template("{prefix} [{wide_bar:.green}] {pos:>3}/{len:3}").unwrap();

  let bar =
    ProgressBar::new(100)
    .with_style(bar_style)
    .with_finish(indicatif::ProgressFinish::Abandon);

  for selection in selections {
    for input in selection.rename_files() {
      let input_file = &input.path;
      let output_file = selection.encode_dir().path.join(&input.mp4_file);

      bar.set_prefix(input.mkv_file.clone());
      // Print this when --verbose is on
      // println!("calling: handbrakecli --json --preset-import-file {} -Z {} -i {} -o {}", profile_file, profile_name, input_file.to_string_lossy(), output_file.to_string_lossy());

      let mut handbrake =
        cmd
          .arg("--json")
          .arg("-i")
          .arg(input_file)
          .arg("-o")
          .arg(output_file)
          .stdout(Stdio::piped())
          .stderr(Stdio::null())
          .spawn()
          .expect("Failed to spawn handbrakecli");

      use std::io::{BufReader, BufRead};
      let out = handbrake.stdout.take().unwrap();
      let stdout_reader = BufReader::new(out);
      let lines = stdout_reader.lines();

      // println!("------------- {}", "before");
      for line in lines {
        match parse(line.unwrap()) {
          Output::Progress(progress) => {
            bar.set_position(progress as u64)
          },
          Output::Ignore => (),
          Output::Done(_) => {
            bar.finish_and_clear()
          }
        }
      }
      // println!("------------- {}", "after");

      let exit_status = handbrake.wait().expect("Could not get output");

      // Use this when there is an error
      let _code = exit_status.code().expect("Could not get exit code");
      // println!("handbrake returned exit code: {}", code);
    }
  }

  Ok(())
}

fn show_select<'a, T: ToString>(options: &'a [T], prompt: &str) -> Result<&'a T, String> {
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
      })
}

