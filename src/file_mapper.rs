use std::path::Path;
use walkdir::WalkDir;
use regex::Regex;
use crate::debug::*;
use std::sync::LazyLock;
use crate::models::SessionToEncodeDir;
use crate::models::RenameTypes;
use crate::models::EntryType;
use crate::models::EncodeDirType;

static RENAME_TV_SERIES_FILE_REG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(session\d{1,})/renames/((S\d{2,}E\d{2,}(?:-E\d{2,})?)\s-\s(.+.mkv))$").unwrap());
static RENAME_MOVIE_FILE_REG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(session\d{1,})/renames/(.+.mkv)$").unwrap());
static ENCODE_FILE_REG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(session\d{1,})/renames/encode_dir\.txt$").unwrap());
static ENCODE_TV_SERIES_DIR_REG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r".+/(.+\s\{tvdb\-\d{1,}\}/Season\s\d{2,})$").unwrap());
static ENCODE_MOVIE_DIR_REG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r".+/(.+\s\{tvdb\-\d{1,}\})$").unwrap());

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
        e @ EntryType::MovieRename { .. } => Some(e),
        e @ EntryType::MovieEncode { .. } => Some(e),
        EntryType::UnknownFileType { .. } => None,
        EntryType::InvalidEncodeDirPath { .. } => None,
      }
    })
    .collect();

  dump_entry_types(&entry_types, verbose);
  dump_unmatched_entry_types(&all_entry_types, verbose);

  let (tv_series_session, movies_session) =
      entry_types
        .iter()
        .filter_map(|entry_type| {
          <EntryType as TryInto<RenameTypes>>::try_into(entry_type.clone())
            .ok()
          })
        .collect();

  dump_mixed_sessions(&tv_series_session, &movies_session, verbose);
  dump_tv_sessions_hash(&tv_series_session, verbose);
  dump_movie_sessions_hash(&movies_session, verbose);

  let (tv_series_encode_dir, movie_encode_dir) =
      entry_types
      .into_iter()
      .filter_map(|entry_type| {
        <EntryType as TryInto<EncodeDirType>>::try_into(entry_type.clone())
          .ok()
        })
      .collect();

  dump_tv_series_encodes_hash(&tv_series_encode_dir, verbose);
  dump_movie_encodes_hash(&movie_encode_dir, verbose);

  let tv_series_session_to_encode_dir = SessionToEncodeDir::from_tvseries_elements(&tv_series_session, &tv_series_encode_dir);
  let movie_session_to_encode_dir = SessionToEncodeDir::from_movie_elements(&movies_session, &movie_encode_dir);

  dump_unmapped_tv_series_sessions_and_encode_dirs(
    &tv_series_session_to_encode_dir,
    &tv_series_session,
    &tv_series_encode_dir,
    verbose
  );

  dump_unmapped_movie_sessions_and_encode_dirs(
    &movie_session_to_encode_dir,
    &movies_session,
    &movie_encode_dir,
    verbose
  );

  let sessions_to_encode_dir: Vec<SessionToEncodeDir> =
    tv_series_session_to_encode_dir
      .into_iter()
      .chain(movie_session_to_encode_dir.into_iter())
      .collect();

  dump_sessions_to_encode_dirs(&sessions_to_encode_dir, verbose);

  sessions_to_encode_dir
}


fn handle_tv_series_rename(path: &Path) -> Option<EntryType> {
    path
      .to_str()
      .and_then(|path| RENAME_TV_SERIES_FILE_REG.captures(path))
      .map(|c| c.extract())
      .map(|(_, [session, file, episode, _])| {
        EntryType::new_tv_series_rename(path, session, episode, file)
      })
}

fn handle_movie_rename(path: &Path) -> Option<EntryType> {
  path
    .to_str()
    .and_then(|path| RENAME_MOVIE_FILE_REG.captures(path))
    .map(|c| c.extract())
    .map(|(_, [session, file])| {
      EntryType::new_movie_rename::<_>(path, session, file)
    })
}

fn handle_encode_file(path: &Path) -> Option<EntryType> {
    path
      .to_str()
      .and_then(|path| ENCODE_FILE_REG.captures(path))
      .map(|c| c.extract())
      .and_then(|(_, [session])| {
        std::fs::read_to_string(path)
          .ok()
          .map(|encode_file_contents| encode_file_contents.trim().to_owned()) // remove newline added by read_to_string
          .and_then(|encode_file_contents| {
            let encode_dir = Path::new(&encode_file_contents);
            if encode_dir.is_dir() && ENCODE_TV_SERIES_DIR_REG.is_match(&encode_file_contents) {
              handle_tv_series_encode_file(&encode_file_contents, session)
            } else if encode_dir.is_dir() && ENCODE_MOVIE_DIR_REG.is_match(&encode_file_contents) {
              handle_movie_encode_file(&encode_file_contents, session)
            } else {
              Some(EntryType::could_not_match_defined_encode_dir(&encode_file_contents))  // Not a directory or encode_dir_reg did not match
            }
          })
      })
}

fn handle_movie_encode_file(contents: &str, session: &str) -> Option<EntryType> {
  ENCODE_MOVIE_DIR_REG
    .captures(contents)
    .map(|c| c.extract())
    .map(|(_, [movie_name])| {
      EntryType::new_movie_encodes(contents, movie_name, session)
    })
}

fn handle_tv_series_encode_file(contents: &str, session: &str) -> Option<EntryType> {
  ENCODE_TV_SERIES_DIR_REG
    .captures(contents)
    .map(|c| c.extract())
    .map(|(_, [season])| {
      EntryType::new_tv_series_encodes(contents, season, session)
    })
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::{LineWriter, Write}};

    use crate::models::{MovieEncodeDir, MovieName, MovieRenameFile, MovieSession, SessionId, TVSeriesEncodeDir, TVSeriesRenameFile, TVSeriesSession};

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
    fn tv_series_regex_with_multiple_episodes_match() {
      let path = "/Some/Path/Rips/session1/renames/S01E04-E05 - The Saga.mkv";
      assert_eq!(RENAME_TV_SERIES_FILE_REG.is_match(path), true);

      let (considered_str, [session, file, episode, _]) = RENAME_TV_SERIES_FILE_REG.captures(path).unwrap().extract();
      assert_eq!(considered_str, "session1/renames/S01E04-E05 - The Saga.mkv");
      assert_eq!(session, "session1");
      assert_eq!(file, "S01E04-E05 - The Saga.mkv");
      assert_eq!(episode, "S01E04-E05");
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

    // ----------------------------------------------------------------------------------------
    // Partial integration test - tests the get_session_encode_mapping using the filesystem.
    // ----------------------------------------------------------------------------------------

    #[test]
    fn gets_tv_series_to_encode() {
      let test_path = get_source_directory("tv_series");

      let encode_dir_content_path = "Encodes/ThunderCats {tvdb-70355}/Season 01";

      create_encode_dir_file(&test_path, encode_dir_content_path, "Rips/session1/renames");

      let session_to_encode_dirs: Vec<SessionToEncodeDir> =
        get_session_encode_mapping(&test_path, false)
          .into_iter()
          .map(|v| v.sorted_files())
          .collect();

      let session_id = SessionId::new("session1");

      let tv_series_rename_files =
        vec![
          TVSeriesRenameFile {
            path: format!("{}/Rips/session1/renames/S01E01 - Exodus.mkv", &test_path).into(),
            session: session_id.clone(),
            episode: "S01E01".to_string(),
            mkv_file: "S01E01 - Exodus.mkv".to_string(),
            mp4_file: "S01E01 - Exodus.mp4".to_string(),
          },
          TVSeriesRenameFile {
            path: format!("{}/Rips/session1/renames/S01E02 - The Unholy Alliance.mkv", &test_path).into(),
            session: session_id.clone(),
            episode: "S01E02".to_string(),
            mkv_file: "S01E02 - The Unholy Alliance.mkv".to_string(),
            mp4_file: "S01E02 - The Unholy Alliance.mp4".to_string(),
          },
          TVSeriesRenameFile {
            path: format!("{}/Rips/session1/renames/S01E03 - Berbils.mkv", &test_path).into(),
            session: session_id.clone(),
            episode: "S01E03".to_string(),
            mkv_file: "S01E03 - Berbils.mkv".to_string(),
            mp4_file: "S01E03 - Berbils.mp4".to_string(),
          },
          TVSeriesRenameFile {
            path: format!("{}/Rips/session1/renames/S01E04-E05 - The Saga.mkv", &test_path).into(),
            session: session_id.clone(),
            episode: "S01E04-E05".to_string(),
            mkv_file: "S01E04-E05 - The Saga.mkv".to_string(),
            mp4_file: "S01E04-E05 - The Saga.mp4".to_string(),
          },
        ];

      let tv_series_encode_dir =
        TVSeriesEncodeDir {
          path: format!("{}/{}", &test_path, encode_dir_content_path).into(),
          season: "ThunderCats {tvdb-70355}/Season 01".into(),
          session_id: session_id.clone(),
        };


      let tv_series_session =
        TVSeriesSession::new(session_id.clone(), tv_series_rename_files);

      let expected =
        vec![
          SessionToEncodeDir::new_tv_series_encode_dir(
            session_id.clone(), tv_series_session, tv_series_encode_dir
          )
        ];

      assert_eq!(session_to_encode_dirs, expected)
    }

    fn create_encode_dir_file(test_path: &str, encode_dir_content_path: &str, session_path: &str) {
      let encode_file_name = Path::new(test_path).join(session_path).join("encode_dir.txt");
      let mut encode_file = File::create(&encode_file_name).expect(&format!("Could not create file: {}", encode_file_name.to_string_lossy()));
      let buf: String = Path::new(test_path).join(encode_dir_content_path).to_string_lossy().to_string();
      encode_file.write_all(buf.as_bytes()).expect("Could not write encode file content");
      encode_file.flush().expect("Could not flush encode file write");
    }

    #[test]
    fn gets_movie_to_encode() {
      let test_path = get_source_directory("movie");
      let encode_dir_content_path = "Encodes/Star Wars - {tvdb-71}";
      create_encode_dir_file(&test_path, encode_dir_content_path, "Rips/session5/renames");

      let session_to_encode_dirs: Vec<SessionToEncodeDir> =
        get_session_encode_mapping(&test_path, true)
          .into_iter()
          .map(|v| v.sorted_files())
          .collect();

      let session_id = SessionId::new("session5");

      let movie_rename_files =
        vec![
          MovieRenameFile {
            path: format!("{}/Rips/session5/renames/Star Wars - {{tvdb-71}}.mkv", &test_path).into(),
            session: session_id.clone(),
            mkv_file: "Star Wars - {tvdb-71}.mkv".to_string(),
            mp4_file: "Star Wars - {tvdb-71}.mp4".to_string(),
          }
        ];

      let movie_encode_dir =
        MovieEncodeDir {
          path: format!("{}/{}", &test_path, &encode_dir_content_path).into(),
          session_id: session_id.clone(),
          movie_name: MovieName::new("Star Wars - {tvdb-71}")
        };


      let movie_session =
        MovieSession::new(session_id.clone(), movie_rename_files);

      let expected =
        vec![
          SessionToEncodeDir::new_movie_encode_dir(
            session_id.clone(), movie_session, movie_encode_dir
          )
        ];

      assert_eq!(session_to_encode_dirs, expected)
    }

    fn get_source_directory(test_directory: &str) -> String {
      let current_directory = std::env::current_dir().expect("Could not get current directory");
      println!("current directory {}", current_directory.to_string_lossy());

      let source_directory = current_directory.join(format!("data/{}", test_directory));
      println!("source directory {}", source_directory.to_string_lossy());

      source_directory.to_string_lossy().to_string()
    }
}
