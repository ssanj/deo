use std::path::{Path, PathBuf};
use std::fmt;

#[derive(Debug, Clone)]
pub enum EntryType {
  Rename {

    /// Full path to mkv file
    path: PathBuf,

    /// Session id of file
    session: SessionId,

    /// Episode
    episode: String,

    /// file name and extension
    file: String
  },

  Encode {

    /// Session id of files that map to this encode directory
    session: SessionId,

    /// Full path to encode directory
    path: PathBuf,

    /// Season
    season: String,
  },
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

#[derive(Debug, Clone)]
pub struct RenameFile {

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

impl TryFrom<EntryType> for RenameFile {
  type Error = ();

  fn try_from(value: EntryType) -> Result<Self, Self::Error> {
    match value {
      EntryType::Rename { path, session, episode, file } => {
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

#[derive(Debug)]
pub struct EncodeDir {
  pub path: PathBuf,
  pub season: String,
  pub session_id: SessionId,
}

impl TryFrom<EntryType> for EncodeDir {
    type Error = ();

    fn try_from(value: EntryType) -> Result<Self, Self::Error> {
      match value {
        EntryType::Encode { path, season, session } => {
          let session_id = session;
          Ok(
            EncodeDir {
              path,
              season,
              session_id
            }
          )
        },
        _ => Err(()),
    }
  }
}


impl EntryType {
  pub fn new_rename<P: AsRef<Path>>(path: P, session: &str, episode: &str, file: &str) -> Self {
    EntryType::Rename {
      path: path.as_ref().to_owned(),
      session: SessionId::new(session),
      episode: episode.to_owned(),
      file: file.to_owned()
    }
  }

  pub fn new_encodes<P: AsRef<Path>>(path: P, season: &str, session: &str) -> Self {
    EntryType::Encode {
      path: path.as_ref().to_owned(),
      season: season.to_owned(),
      session: SessionId::new(session)
    }
  }
}
