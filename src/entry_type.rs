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

pub trait SessionTypeAware {
  fn session_id(&self) -> SessionId;
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

/// Convert from a collection of RenameFile into a Map<SessionId, Session>
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


#[derive(Debug, Clone)]
pub enum EncodeDirType {
  TVSeries(TVSeriesEncodeDir),
  Movie(MovieEncodeDir),
}

pub trait EncodeDirPathAware {
  fn path(&self) -> PathBuf;
}

pub trait LocationAware {
  fn location(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct TVSeriesEncodeDir {
  pub path: PathBuf,
  pub season: String,
  pub session_id: SessionId,
}

#[derive(Debug, Clone)]
pub struct MovieEncodeDir {
  pub path: PathBuf,
  pub session_id: SessionId,
  pub movie_name: MovieName
}

impl TryFrom<EntryType> for EncodeDirType {
    type Error = ();

    fn try_from(value: EntryType) -> Result<Self, Self::Error> {
      match value {
        EntryType::TVSeriesEncode { path, season, session } => {
          let session_id = session;
          Ok(
            EncodeDirType::TVSeries(
              TVSeriesEncodeDir {
                path,
                season,
                session_id
              }
            )
          )
        },
        EntryType::MovieEncode { path, session, movie_name } => {
          let session_id = session;
          Ok(
            EncodeDirType::Movie(
              MovieEncodeDir {
                path,
                session_id,
                movie_name
              }
            )
          )
        },
        _ => {
          Err(())
        },
    }
  }
}

impl SessionTypeAware for EncodeDirType {
    fn session_id(&self) -> SessionId {
      match self {
        EncodeDirType::TVSeries(tvseries_encode_dir) => tvseries_encode_dir.session_id.clone(),
        EncodeDirType::Movie(movie_encode_dir) => movie_encode_dir.session_id.clone(),
      }
    }
}

impl EncodeDirPathAware for EncodeDirType {
    fn path(&self) -> PathBuf {
        match self {
            EncodeDirType::TVSeries(tvseries_encode_dir) => tvseries_encode_dir.path.clone(),
            EncodeDirType::Movie(movie_encode_dir) => movie_encode_dir.path.clone(),
        }
    }
}


impl LocationAware for EncodeDirType {
    fn location(&self) -> String {
      match self {
        EncodeDirType::TVSeries(tvseries_encode_dir) => tvseries_encode_dir.season.clone(),
        EncodeDirType::Movie(movie_encode_dir) => movie_encode_dir.movie_name.to_string(),
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
  files: Vec<RenameTypes>,
}

impl Session {

  pub fn new(session_id: SessionId, files: Vec<RenameTypes>) -> Self {
    Self {
      session_id,
      files
    }
  }

  pub fn files(&self) -> Vec<RenameTypes> {
    let mut sorted_files = self.files.clone();
    sorted_files.sort_by(|a, b| a.mkv_file().cmp(&b.mkv_file()));
    sorted_files
  }
}

#[derive(Debug)]
pub struct SessionToEncodeDir {
  session_id: SessionId,
  session: Session,
  encode_dir: EncodeDirType
}

impl SessionToEncodeDir {
  pub fn new_tv_series_encode_dir(session_id: SessionId, session: Session, encode_dir: TVSeriesEncodeDir) -> Self {
    Self {
      session_id,
      session,
      encode_dir: EncodeDirType::TVSeries(encode_dir)
    }
  }

  pub fn new_movie_encode_dir(session_id: SessionId, session: Session, encode_dir: MovieEncodeDir) -> Self {
    Self {
      session_id,
      session,
      encode_dir: EncodeDirType::Movie(encode_dir)
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

  pub fn encode_dir(&self) -> &EncodeDirType {
    &self.encode_dir
  }
}
