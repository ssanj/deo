use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use super::movie::MovieSession;
use super::tv_series::TVSeriesSession;
use super::SessionId;
use super::EntryType;

#[derive(Debug, Clone)]
pub enum RenameTypes {
  TVSeries(TVSeriesRenameFile),
  Movie(MovieRenameFile)
}


#[derive(Debug, Clone)]
pub struct TVSeriesRenameFile {

  /// Full path to mkv_file
  pub path: PathBuf,

  /// Session id associated with file
  pub session: SessionId,

  /// Episode name
  pub episode: String,

  /// Input file name and ext - file to be encoded
  pub mkv_file: String,

  /// Output file and ext - encoded file
  pub mp4_file: String,
}

#[derive(Debug, Clone)]
pub struct MovieRenameFile {

  /// Full path to mkv_file
  pub path: PathBuf,

  /// Session id associated with file
  pub session: SessionId,

  /// Input file name and ext - file to be encoded
  pub mkv_file: String,

  /// Output file and ext - encoded file
  pub mp4_file: String,
}

/// Convert from a collection of RenameFile into a Map<SessionId, TVSeriesSession>
impl FromIterator<RenameTypes> for (HashMap<SessionId, TVSeriesSession>, HashMap<SessionId, MovieSession>) {

  fn from_iter<T: IntoIterator<Item = RenameTypes>>(renames: T) -> Self {
    let mut hash: HashMap<SessionId, (Vec<TVSeriesRenameFile>, Vec<MovieRenameFile>)> = HashMap::new();
      for rename in renames {
        match rename {
            RenameTypes::TVSeries(tvseries_rename_file) => {
              hash
                .entry(tvseries_rename_file.clone().session)
                .and_modify(|v| v.0.push(tvseries_rename_file.clone()))
                .or_insert((vec![tvseries_rename_file], vec![]));
            },
            RenameTypes::Movie(movie_rename_file) => {
              hash
                .entry(movie_rename_file.clone().session)
                .and_modify(|v| v.1.push(movie_rename_file.clone()))
                .or_insert((vec![], vec![movie_rename_file]));
            }
        }
      }

      let tv_renames_hash: HashMap<SessionId, TVSeriesSession> =
        hash
          .clone()
          .into_iter()
          .map(|(k, v)| (k.clone(), TVSeriesSession::new(k, v.0)))
          .collect();

      let movie_renames_hash: HashMap<SessionId, MovieSession> =
        hash
          .into_iter()
          .map(|(k, v)| (k.clone(), MovieSession::new(k, v.1)))
          .collect();

      (tv_renames_hash, movie_renames_hash)
    }
}


impl TryFrom<EntryType> for RenameTypes {
  type Error = ();

  fn try_from(value: EntryType) -> Result<Self, Self::Error> {
    match value {
      EntryType::TVSeriesRename { path, session, episode, file } => {
        let output_path = Path::new(&file);

        let mp4_file =
          output_path
            .file_stem()
            .map(|f| format!("{}.mp4", f.to_string_lossy()))
            .expect("Could not get file stem");

        let mkv_file = file;

        Ok(
          RenameTypes::TVSeries(TVSeriesRenameFile {
            path,
            session,
            episode,
            mkv_file,
            mp4_file,
          }
        ))
      },
      EntryType::MovieRename { path, session, file } => {
        let output_path = Path::new(&file);

        let mp4_file =
          output_path
            .file_stem()
            .map(|f| format!("{}.mp4", f.to_string_lossy()))
            .expect("Could not get file stem");

        let mkv_file = file;

        Ok(
          RenameTypes::Movie(MovieRenameFile {
            path,
            session,
            mkv_file,
            mp4_file,
          }
        ))
      },
      _ => Err(()),
    }
  }
}
