use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use crate::user_selection::UserSelection;
use crate::hb_output_parser::{parse, Output};

pub fn encode(selections: Vec<UserSelection>) -> Result<(), String> {
  println!("encoding...");
  let mut cmd = Command::new("handbrakecli");

  let multi = MultiProgress::new();

  let bar_style =
    ProgressStyle::with_template("pass:{msg} {prefix} [{wide_bar:.green}] {pos:>3}/{len:3} {eta}").unwrap();

  let bar =
    ProgressBar::new(100)
    .with_style(bar_style)
    .with_finish(indicatif::ProgressFinish::Abandon);

  let completed_bar_style =
    ProgressStyle::with_template("completed:{pos:>3}/{len:3} [{wide_bar:.blue}] {eta}").unwrap();

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
    ProgressStyle::with_template("errors:{pos:>3}/{len:3} [{wide_bar:.160}]").unwrap();

  let error_bar =
    ProgressBar::new(file_count)
      .with_style(error_bar_style)
      .with_finish(indicatif::ProgressFinish::Abandon);

  multi.add(bar.clone());
  multi.add(completed_bar.clone());
  multi.add(error_bar.clone());

  completed_bar.set_position(0);
  error_bar.set_position(0);

  let log_file_path = Path::new("dea.log");
  if !selections.is_empty() && log_file_path.exists() {
    std::fs::remove_file(log_file_path)
      // TODO: Add stronger types around this error
      .map_err(|e| format!("Could not remove log file: deo.log due to: {}", e.to_string()))?
  }

  // TODO: Make this strongly typed
  let mut log_file =
    OpenOptions::new()
      .create_new(true)
      .append(true)
      .open(log_file_path)
      .map_err(|e| format!("Could not open log_file: deo.log due to: {}", e.to_string()))?;

  for selection in selections {
    for input in selection.rename_files() {
      bar.set_message("0");
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
          Output::Pass(pass) => {
            bar.set_message(pass.to_string())
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
        error_bar.inc(1);
        log_file.write_all(&format!("{} ❌\n", input_file.to_string_lossy()).into_bytes()).unwrap();
      } else {
        log_file.write_all(&format!("{} ✅\n", input_file.to_string_lossy()).into_bytes()).unwrap();
      }

      completed_bar.inc(1);
      log_file.flush().unwrap();
    }

  }

  Ok(())
}
