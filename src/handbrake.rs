use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::process::{Command, Stdio};

use crate::user_selection::UserSelection;
use crate::hb_output_parser::{parse, Output};


pub fn encode(selections: Vec<UserSelection>) -> Result<(), String> {
  println!("encoding...");
  //handbreakcli
  // --preset-import-file ~/Desktop/DVD\ -\ H265\ Apple\ Silicon\ HQ.json
  // -Z "DVD - H265 Apple Silicon HQ"
  // -i S05E01\ -\ Mr.\ Monk\ and\ the\ Actor.mkv
  // -o S05E01\ -\ Mr.\ Monk\ and\ the\ Actor.mp4

  let mut cmd = Command::new("handbrakecli");

  let multi = MultiProgress::new();

  let bar_style =
    ProgressStyle::with_template("{prefix} [{wide_bar:.green}] {pos:>3}/{len:3} {eta}").unwrap();

  let bar =
    ProgressBar::new(100)
    .with_style(bar_style)
    .with_finish(indicatif::ProgressFinish::Abandon);

  let completed_bar_style =
    ProgressStyle::with_template("{prefix} {pos}/{len} [{wide_bar:.blue}] {pos:>3}/{len:3} {eta}").unwrap();

  let file_count =
    selections
      .iter()
      .map(|sel| {
        sel.rename_files().len() as u64
      })
      .sum();


  let completed_bar =
    ProgressBar::new(file_count)
    .with_style(completed_bar_style)
    .with_finish(indicatif::ProgressFinish::Abandon);

  let error_bar_style =
    ProgressStyle::with_template("{prefix} {pos}/{len} [{wide_bar:.red}] {pos:>3}/{len:3}").unwrap();

  let error_bar =
    ProgressBar::new(file_count)
      .with_style(error_bar_style)
      .with_finish(indicatif::ProgressFinish::Abandon);

  multi.add(bar.clone());
  multi.add(completed_bar.clone());
  multi.add(error_bar.clone());

  completed_bar.set_prefix("completed: ");
  error_bar.set_prefix("errors: ");

  for selection in selections {
    for input in selection.rename_files() {
      let input_file = &input.path;
      let output_file = selection.encode_dir().path.join(&input.mp4_file);
      bar.set_prefix(input.mkv_file.clone());
      // Print this when --verbose is on
      // println!("calling: handbrakecli --json --preset-import-file {} -Z {} -i {} -o {}", profile_file, profile_name, input_file.to_string_lossy(), output_file.to_string_lossy());

      let profile = selection.profile();

      let mut handbrake =
        cmd
          .arg("--preset-import-file")
          .arg(profile.full_path())
          .arg("-Z")
          .arg(profile.preset_name())
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
      let exit_status = handbrake.wait().expect("Could not get output");

      // Write exit code to a file with the file name so we can identify encoding errors
      let code = exit_status.code().expect("Could not get exit code");
      // println!("handbrake returned exit code: {}", code);
      if code != 0 {
        error_bar.inc(1)
      }
      completed_bar.inc(1);
    }

  }

  Ok(())
}
