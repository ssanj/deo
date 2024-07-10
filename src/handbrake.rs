use indicatif::{ProgressBar, ProgressStyle};
use std::process::{Command, Stdio};

use crate::user_selection::UserSelection;
use crate::hb_output_parser::{parse, Output};


pub fn encode(selections: Vec<UserSelection>) -> Result<(), String> {
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
      let _code = exit_status.code().expect("Could not get exit code");
      // println!("handbrake returned exit code: {}", code);
    }
  }

  Ok(())
}
