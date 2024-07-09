use std::path::{Path, PathBuf};
use std::fmt;

#[derive(Debug, Clone)]
pub enum EntryType {
  Session {
    path: PathBuf,
    session: SessionId,
    episode: String,
    file: String
  },
  Encode {
    session: SessionId,
    path: PathBuf,
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
pub struct SessionType {
    pub path: PathBuf,
    pub session: SessionId,
    pub episode: String,
    pub input_file: String,
    pub output_file: String,
    pub file_name: String,
  }

impl TryFrom<EntryType> for SessionType {
  type Error = ();

  fn try_from(value: EntryType) -> Result<Self, Self::Error> {
    match value {
      EntryType::Session { path, session, episode, file } => {
        let output_path = Path::new(&file);

        let output_file =
          output_path
            .file_stem()
            .map(|f| format!("{}.mp4", f.to_string_lossy()))
            .expect("Could not get file stem");


        let file_name = output_path.file_name().map_or_else(|| "<unknown>".to_owned(), |v| v.to_string_lossy().to_string());

        let input_file = file;

        Ok(
          SessionType {
            path,
            session,
            episode,
            input_file,
            output_file,
            file_name
          }
        )
      },
      _ => Err(()),
    }
  }
}

#[derive(Debug)]
pub struct EncodeType {
  pub path: PathBuf,
  pub season: String,
  pub session_id: SessionId,
}

impl TryFrom<EntryType> for EncodeType {
    type Error = ();

    fn try_from(value: EntryType) -> Result<Self, Self::Error> {
      match value {
        EntryType::Encode { path, season, session } => {
          let session_id = session;
          Ok(
            EncodeType {
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
  pub fn new_session<P: AsRef<Path>>(path: P, session: &str, episode: &str, file: &str) -> Self {
    EntryType::Session {
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
