use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;
use regex::Regex;
use crate::debug::*;
use std::sync::LazyLock;
use crate::entry_type::{EncodeDir, EntryType, RenameFile, Session, SessionId, SessionToEncodeDir};

static RENAME_TV_SERIES_FILE_REG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(session\d{1,})\/renames\/((S\d{2,}E\d{2,})\s-\s(.+.mkv))$").unwrap());
static RENAME_MOVIE_FILE_REG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(session\d{1,})\/renames\/(.+.mkv)$").unwrap());
static ENCODE_FILE_REG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(session\d{1,})\/renames\/encode_dir\.txt$").unwrap());
static ENCODE_TV_SERIES_DIR_REG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r".+\/(.+\s\{tvdb\-\d{1,}\}\/Season\s\d{2,})$").unwrap());
static ENCODE_MOVIE_DIR_REG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r".+\/(.+\s\{tvdb\-\d{1,}\})$").unwrap());

pub fn get_session_encode_mapping<P: AsRef<Path>>(source: P, verbose: bool) -> Vec<SessionToEncodeDir> {
  let all_entry_types: Vec<EntryType> =
    WalkDir::new(source)
      .into_iter()
      .filter_map(|de| de.ok())
      .filter_map(|de| {
        if de.file_type().is_file() && RENAME_TV_SERIES_FILE_REG.is_match(de.path().to_str().unwrap()){
          handle_tv_series_rename(de.path())
        }  else if de.file_type().is_file() && RENAME_MOVIE_FILE_REG.is_match(de.path().to_str().unwrap()){
          handle_movie_rename(de.path())
        } else if de.file_type().is_file() && ENCODE_FILE_REG.is_match(de.path().to_str().unwrap()) {
          handle_encode_file(de.path())
        } else {
          Some(EntryType::unknown_file_type(de.path())) // Not a file or encode_file_reg did not match
        }
      })
      .collect();

  let entry_types: Vec<EntryType> =
    all_entry_types
    .iter()
    .cloned()
    .filter_map(|et| {
      match et {
        r @ EntryType::TVSeriesRename { .. } => Some(r),
        e @ EntryType::TVSeriesEncode { .. } => Some(e),
        e @ EntryType::MovieEncode { .. } => Some(e),
        e @ EntryType::MovieRename { .. } => Some(e),
        EntryType::UnknownFileType { .. } => None,
        EntryType::InvalidEncodeDirPath { .. } => None,
      }
    })
    .collect();

  dump_entry_types(&entry_types, verbose);

  dump_unmatched_entry_types(&all_entry_types, verbose);

  let sessions_hash: HashMap<SessionId, Session> =
    entry_types
      .iter()
      .filter_map(|session_type| {
        <EntryType as TryInto<RenameFile>>::try_into(session_type.clone())
          .ok()
        })
      .collect();


  dump_sessions_hash(&sessions_hash, verbose);

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

  dump_encodes_hash(&encode_dir_hash, verbose);

  let mut sessions_to_encode_dir: Vec<SessionToEncodeDir> = vec![];

  // Map from SessionId -> SessionToEncodeDir
  // TODO: Can we map over this?
  for (session_id, session) in sessions_hash.iter() {
    if let Some(encode_dir) = encode_dir_hash.get(session_id) {
      sessions_to_encode_dir.push(SessionToEncodeDir::new(session_id.clone(), session.clone(), encode_dir.clone()))
    }
  }

  dump_sessions_to_encode_dirs(&sessions_to_encode_dir, verbose);

  dump_unmapped_sessions_and_encode_dirs(&sessions_to_encode_dir, &sessions_hash, &encode_dir_hash, verbose);

  sessions_to_encode_dir
}


fn handle_tv_series_rename(path: &Path) -> Option<EntryType> {
    if let Some((_, [session, file, episode, _])) = RENAME_TV_SERIES_FILE_REG.captures(path.to_str().unwrap()).map(|c| c.extract()) {
      Some(EntryType::new_tv_series_rename(path, session, episode, file))
    } else {
      None
    }
}

fn handle_movie_rename(path: &Path) -> Option<EntryType> {
    if let Some((_, [session, file])) = RENAME_MOVIE_FILE_REG.captures(path.to_str().unwrap()).map(|c| c.extract()) {
      Some(EntryType::new_movie_rename::<_>(path, session, file))
    } else {
      None
    }
}

fn handle_encode_file(path: &Path) -> Option<EntryType> {
    if let Some((_, [session])) = ENCODE_FILE_REG.captures(path.to_str().unwrap()).map(|c| c.extract()) {
      std::fs::read_to_string(path)
        .ok()
        .map(|encode_file_contents| encode_file_contents.trim().to_owned()) // remove newline added by read_to_string
        .and_then(|encode_file_contents| {
          let encode_dir = Path::new(&encode_file_contents);
          if encode_dir.is_dir() && ENCODE_TV_SERIES_DIR_REG.is_match(&encode_file_contents) {
            handle_tv_series_encode_file(&encode_file_contents, session)
          } else {
            Some(EntryType::could_not_match_defined_encode_dir(&encode_file_contents))  // Not a directory or encode_dir_reg did not match
          }
        })
    } else {
      None
    }
}

fn handle_tv_series_encode_file(contents: &str, session: &str) -> Option<EntryType> {
    if let Some((_, [season])) = ENCODE_TV_SERIES_DIR_REG.captures(contents).map(|c| c.extract()) {
      Some(EntryType::new_encodes(contents, season, session))
    } else {
      None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn tv_series_regex_match() {
      let path = "/Some/Path/Rips/session1/renames/S01E02 - The Unholy Alliance.mkv";
      assert_eq!(RENAME_TV_SERIES_FILE_REG.is_match(path), true);

      let (considered_str, [session, file, episode, _]) = RENAME_TV_SERIES_FILE_REG.captures(path).unwrap().extract();
      assert_eq!(considered_str, "session1/renames/S01E02 - The Unholy Alliance.mkv");
      assert_eq!(session, "session1");
      assert_eq!(file, "S01E02 - The Unholy Alliance.mkv");
      assert_eq!(episode, "S01E02");
    }

    #[test]
    fn movie_regex_match() {
      let path = "/Some/Path/Rips/session3/renames/Return of the Jedi - {tvdb-698}.mkv";
      assert_eq!(RENAME_MOVIE_FILE_REG.is_match(path), true);

      let (considered_str, [session, file]) = RENAME_MOVIE_FILE_REG.captures(path).unwrap().extract();
      assert_eq!(considered_str, "session3/renames/Return of the Jedi - {tvdb-698}.mkv");
      assert_eq!(session, "session3");
      assert_eq!(file, "Return of the Jedi - {tvdb-698}.mkv");
    }

    #[test]
    fn encode_file_name_regex_match() {
      let encode_file_path = "/Some/Path/Rips/session2/renames/encode_dir.txt";
      assert_eq!(ENCODE_FILE_REG.is_match(encode_file_path), true);

      let (considered_str, [session]) = ENCODE_FILE_REG.captures(&encode_file_path).unwrap().extract();
      assert_eq!(considered_str, "session2/renames/encode_dir.txt");
      assert_eq!(session, "session2")
    }

    #[test]
    fn encode_file_tv_series_contents_regex_match() {
      let encode_file_contents = "/Some/Path/Encodes/ThunderCats {tvdb-70355}/Season 01";
      assert_eq!(ENCODE_TV_SERIES_DIR_REG.is_match(encode_file_contents), true);

      let (considered_str, [encodes_dir]) = ENCODE_TV_SERIES_DIR_REG.captures(encode_file_contents).unwrap().extract();

      assert_eq!(considered_str, "/Some/Path/Encodes/ThunderCats {tvdb-70355}/Season 01");
      assert_eq!(encodes_dir, "ThunderCats {tvdb-70355}/Season 01");
    }

    #[test]
    fn encode_file_movie_contents_regex_match() {
      let encode_file_contents = "/Some/Path/Encodes/Return of the Jedi - {tvdb-698}";
      assert_eq!(ENCODE_MOVIE_DIR_REG.is_match(encode_file_contents), true);

      let (considered_str, [encodes_dir]) = ENCODE_MOVIE_DIR_REG.captures(encode_file_contents).unwrap().extract();

      assert_eq!(considered_str, "/Some/Path/Encodes/Return of the Jedi - {tvdb-698}");
      assert_eq!(encodes_dir, "Return of the Jedi - {tvdb-698}");
    }
}
