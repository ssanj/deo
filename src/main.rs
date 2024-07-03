use args::cli;
use walkdir::WalkDir;
use regex::Regex;
use entry_type::EntryType;

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

  println!("{}", session_names.join("\n"))
}
