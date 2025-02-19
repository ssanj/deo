use std::path::{Path, PathBuf};
use std::fmt;
use std::collections::HashMap;

// TODO: Extend EntryType to have one rename for tv series and one for movies
// This class models something that can be encoded later.
#[derive(Debug, Clone)]
pub enum EntryType {
  TVSeriesRename {

    /// Full path to mkv file
    path: PathBuf,

    /// Session id of file
    session: SessionId,

    /// Episode
    episode: String,

    /// file name and extension
    file: String
  },

  MovieRename {

    /// Full path to mkv file
    path: PathBuf,

    /// Session id of file
    session: SessionId,

    /// file name and extension
    file: String
  },

  TVSeriesEncode {

    /// Session id of files that map to this encode directory
    session: SessionId,

    /// Full path to encode directory
    path: PathBuf,

    // TODO: What does this mean in terms of a movie? Is it the name? Choose a better name for this field
    /// Season
    season: String,
  },

  MovieEncode {

    /// Session id of files that map to this encode directory
    session: SessionId,

    /// Full path to encode directory
    path: PathBuf,

    /// Name of the movie
    movie_name: MovieName
  },

  UnknownFileType {
    path: PathBuf
  },


  InvalidEncodeDirPath {
    defined_path: String
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct SessionId(String);

impl SessionId {
  pub fn new(value: &str) -> Self {
    Self(value.to_string())
  }

  pub fn id(&self) -> &str {
    &self.0
  }
}

impl fmt::Display for SessionId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", self.0)
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct MovieName(String);

impl MovieName {
  pub fn new(value: &str) -> Self {
    Self(value.to_string())
  }

  pub fn id(&self) -> &str {
    &self.0
  }
}

impl fmt::Display for MovieName {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", self.0)
  }
}


#[derive(Debug, Clone)]
pub struct RenameFile {

  /// Full path to mkv_file
  pub path: PathBuf,

  /// Session id associated with file
  pub session: SessionId,

  // TODO: Why do we need this except for debugging?
  /// Episode name
  pub episode: String,

  /// Input file name and ext - file to be encoded
  pub mkv_file: String,

  /// Output file and ext - encoded file
  pub mp4_file: String,
}

impl RenameFile {
  pub fn session_id(&self) -> SessionId {
    self.session.clone()
  }
}

/// Convert from a collection of RenameFile into a Map<SessionId, Session>
impl FromIterator<RenameFile> for HashMap<SessionId, Session> {

  fn from_iter<T: IntoIterator<Item = RenameFile>>(renames: T) -> Self {
    let mut hash: HashMap<SessionId, Vec<RenameFile>> = HashMap::new();
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

impl TryFrom<EntryType> for RenameFile {
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
          RenameFile {
            path,
            session,
            episode,
            mkv_file,
            mp4_file,
          }
        )
      },
      _ => Err(()),
    }
  }
}

#[derive(Debug, Clone)]
pub struct EncodeDir {
  pub path: PathBuf,
  // TODO: Why would we need a season for a movie encode?
  pub season: String,
  pub session_id: SessionId,
}

impl TryFrom<EntryType> for EncodeDir {
    type Error = ();

    fn try_from(value: EntryType) -> Result<Self, Self::Error> {
      match value {
        EntryType::TVSeriesEncode { path, season, session } => {
          let session_id = session;
          Ok(
            EncodeDir {
              path,
              season,
              session_id
            }
          )
        },
        _ => {
          Err(())
        },
    }
  }
}


impl EntryType {
  pub fn new_tv_series_rename<P: AsRef<Path>>(path: P, session: &str, episode: &str, file: &str) -> Self {
    EntryType::TVSeriesRename {
      path: path.as_ref().to_owned(),
      session: SessionId::new(session),
      episode: episode.to_owned(),
      file: file.to_owned()
    }
  }

  pub fn new_movie_rename<P: AsRef<Path>>(path: P, session: &str, file: &str) -> Self {
    EntryType::MovieRename {
      path: path.as_ref().to_owned(),
      session: SessionId::new(session),
      file: file.to_owned()
    }
  }

  pub fn new_tv_series_encodes<P: AsRef<Path>>(path: P, season: &str, session: &str) -> Self {
    EntryType::TVSeriesEncode {
      path: path.as_ref().to_owned(),
      season: season.to_owned(),
      session: SessionId::new(session)
    }
  }

  pub fn new_movie_encodes<P: AsRef<Path>>(path: P, movie_name: &str, session: &str) -> Self {
    EntryType::MovieEncode {
      path: path.as_ref().to_owned(),
      movie_name: MovieName::new(movie_name),
      session: SessionId::new(session)
    }
  }

  pub(crate) fn could_not_match_defined_encode_dir(encode_file_contents: &str) -> EntryType {
    EntryType::InvalidEncodeDirPath {
      defined_path: encode_file_contents.to_owned()
    }
  }

  pub(crate) fn unknown_file_type<P: AsRef<Path>>(path: P) -> EntryType {
    EntryType::UnknownFileType {
      path: path.as_ref().to_owned()
    }
  }
}

/// A Session has a SessionId and a list of files (RenameFile)
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Session {
  session_id: SessionId,
  files: Vec<RenameFile>,
}

impl Session {

  pub fn new(session_id: SessionId, files: Vec<RenameFile>) -> Self {
    Self {
      session_id,
      files
    }
  }

  pub fn files(&self) -> Vec<RenameFile> {
    let mut sorted_files = self.files.clone();
    sorted_files.sort_by(|a, b| a.mkv_file.cmp(&b.mkv_file));
    sorted_files
  }
}

#[derive(Debug)]
pub struct SessionToEncodeDir {
  session_id: SessionId,
  session: Session,
  encode_dir: EncodeDir
}

impl SessionToEncodeDir {
  pub fn new(session_id: SessionId, session: Session, encode_dir: EncodeDir) -> Self {
    Self {
      session_id,
      session,
      encode_dir
    }
  }
}

impl SessionToEncodeDir {
  pub fn session_id(&self) -> &SessionId {
    &self.session_id
  }

  pub fn session(&self) -> &Session {
    &self.session
  }

  pub fn encode_dir(&self) -> &EncodeDir {
    &self.encode_dir
  }
}
