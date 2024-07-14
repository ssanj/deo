use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;
use regex::Regex;
use crate::entry_type::{EntryType, EncodeDir, RenameFile, SessionId, Session, SessionToEncodeDir};

pub fn get_session_encode_mapping<P: AsRef<Path>>(source: P) -> Vec<SessionToEncodeDir> {
  // let rename_file_reg = Regex::new(r"(session\d{1,})\/renames\/(S\d{2,}E\d{2,})\s-\s(.+.mkv)$").unwrap();
  let rename_file_reg = Regex::new(r"(session\d{1,})\/renames\/((S\d{2,}E\d{2,})\s-\s(.+.mkv))$").unwrap();
  let encode_file_reg = Regex::new(r"(session\d{1,})\/renames\/encode_dir\.txt$").unwrap();
  let encode_dir_reg = Regex::new(r".+\/(.+\s\{tvdb\-\d{1,}\}\/Season\s\d{2,})$").unwrap();

  let entry_types: Vec<EntryType> =
    WalkDir::new(source)
      .into_iter()
      .filter_map(|de| de.ok())
      .filter_map(|de| {
        if de.file_type().is_file() && rename_file_reg.is_match(de.path().to_str().unwrap()){
          if let Some((_, [session, file, episode, _])) = rename_file_reg.captures(de.path().to_str().unwrap()).map(|c| c.extract()) {
            Some(EntryType::new_rename(de.path(), session, episode, file))
          } else {
            None
          }
        } else if de.file_type().is_file() && encode_file_reg.is_match(de.path().to_str().unwrap()) {
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

  // TODO: Print this on --verbose
  //println!("entry_types: {:?}", &entry_types);

  let sessions_hash: HashMap<SessionId, Session> =
    entry_types
      .iter()
      .filter_map(|session_type| {
        <EntryType as TryInto<RenameFile>>::try_into(session_type.clone())
          .ok()
        })
      .collect();

  // TODO: Print this on --verbose
  //println!("sessions_hash \n{:?}", sessions_hash);

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

  // TODO: Print this on --verbose
  //println!("encode_dir_hash {:?}", &encode_dir_hash);

  let mut sessions_to_encode_dir: Vec<SessionToEncodeDir> = vec![];

  // Map from SessionId -> SessionToEncodeDir
  // TODO: Can we map over this?
  for (session_id, session) in sessions_hash {
    if let Some(encode_dir) = encode_dir_hash.get(&session_id) {
      sessions_to_encode_dir.push(SessionToEncodeDir::new(session_id, session, encode_dir.clone()))
    }
  }

  // TODO: Print this on --verbose
  //println!("mappings {:?}", &sessions_to_encode_dir);
  sessions_to_encode_dir
}
