use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use super::movie::MovieSession;
use super::tv_series::TVSeriesSession;
use super::SessionId;
use super::Session;
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

pub trait SessionTypeAware {
  fn session_id(&self) -> SessionId;
}

pub trait SessionFilesAware {
  fn files(&self) -> Vec<RenameTypes>;
}

pub trait EpisodeName {
  fn episode(&self) -> Option<String>;
}

pub trait MKVTypeAware {
  fn mkv_file(&self) -> String;
  fn mp4_file(&self) -> String;
  fn mkv_path(&self) -> PathBuf;
}


impl SessionTypeAware for RenameTypes {
  fn session_id(&self) -> SessionId {
    match self {
        RenameTypes::TVSeries(tvseries_rename_file) => tvseries_rename_file.session.clone(),
        RenameTypes::Movie(movie_rename_file) => movie_rename_file.session.clone(),
    }
  }
}

impl MKVTypeAware for RenameTypes {
  fn mkv_file(&self) -> String {
    match self {
      RenameTypes::TVSeries(tvseries_rename_file) => tvseries_rename_file.mkv_file.to_owned(),
      RenameTypes::Movie(movie_rename_file) => movie_rename_file.mkv_file.to_owned(),
    }
  }

  fn mp4_file(&self) -> String {
    match self {
      RenameTypes::TVSeries(tvseries_rename_file) => tvseries_rename_file.mp4_file.to_owned(),
      RenameTypes::Movie(movie_rename_file) => movie_rename_file.mp4_file.to_owned(),
    }
  }

  fn mkv_path(&self) -> PathBuf {
    match self {
      RenameTypes::TVSeries(tvseries_rename_file) => tvseries_rename_file.path.clone(),
      RenameTypes::Movie(movie_rename_file) => movie_rename_file.path.clone(),
    }
  }
}

impl MKVTypeAware for TVSeriesRenameFile {
    fn mkv_file(&self) -> String {
      self.mkv_file.clone()
    }

    fn mp4_file(&self) -> String {
        self.mp4_file.clone()
    }

    fn mkv_path(&self) -> PathBuf {
        self.path.clone()
    }
}

impl MKVTypeAware for MovieRenameFile {
    fn mkv_file(&self) -> String {
        self.mkv_file.clone()
    }

    fn mp4_file(&self) -> String {
        self.mp4_file.clone()
    }

    fn mkv_path(&self) -> PathBuf {
        self.path.clone()
    }
}

impl EpisodeName for RenameTypes {
    fn episode(&self) -> Option<String> {
      match self {
        RenameTypes::TVSeries(tvseries_rename_file) => Some(tvseries_rename_file.episode.to_string()),
        RenameTypes::Movie(_) => None,
      }
    }
}

/// Convert from a collection of RenameFile into a Map<SessionId, Session>
//TODO: Remove
impl FromIterator<RenameTypes> for HashMap<SessionId, Session> {

  fn from_iter<T: IntoIterator<Item = RenameTypes>>(renames: T) -> Self {
    let mut hash: HashMap<SessionId, Vec<RenameTypes>> = HashMap::new();
      for rename in renames {
        hash
          .entry(rename.session_id())
          .and_modify(|v| v.push(rename.clone()))
          .or_insert(vec![rename]);
      }

      hash
        .into_iter()
        .map(|(k, v)| (k.clone(), Session::new(k, v)))
        .collect()
    }
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
