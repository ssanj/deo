use std::path::{Path, PathBuf};

use args::cli;
use walkdir::WalkDir;
use regex::Regex;

mod args;

#[derive(Debug)]
enum DirType {
  SessionDir {
    path: PathBuf,
    episode: String,
    file: String
  },
  EncodeDir(PathBuf),
}

impl DirType {
  pub fn new_session_dir<P: AsRef<Path>>(path: P, episode: &str, file: &str) -> Self {
    DirType::SessionDir {
      path: path.as_ref().to_owned(),
      episode: episode.to_owned(),
      file: file.to_owned()
    }
  }

  pub fn new_encodes_dir<P: AsRef<Path>>(path: P) -> Self {
    DirType::EncodeDir(path.as_ref().to_owned())
  }
}

fn main() {
  let args = cli::get_cli_args();
  println!("source: {}", &args.source.to_string_lossy());
  let rename_reg = Regex::new(r"(S?\d{2,}E\d{2,})\s-\s(.+.mkv)$").unwrap();

  let sessions: Vec<DirType> =
    WalkDir::new(&args.source)
      .into_iter()
      .filter_map(|de| de.ok())
      .filter_map(|de| {
        if de.file_type().is_file() && rename_reg.is_match(de.path().to_str().unwrap()){
          if let Some((_, [episode, file])) = rename_reg.captures(de.path().to_str().unwrap()).map(|c| c.extract()) {
            Some(DirType::new_session_dir(de.path(), episode, file))
          } else {
            None
          }
        } else if de.file_type().is_dir() {
          let path = de.path();
          if path.strip_prefix(&args.source).unwrap().starts_with("Encodes") && path.to_string_lossy().contains("-") {
            Some(DirType::new_encodes_dir(de.path()))
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

  println!("{}", session_names.join("\n"))
}
