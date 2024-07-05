use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum EntryType {
  Session {
    path: PathBuf,
    session: String,
    episode: String,
    file: String
  },
  Encode {
    path: PathBuf,
    season: String,
  },
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct SessionId(String);

impl SessionId {
  pub fn new(value: &str) -> Self {
    Self(value.to_string())
  }

  pub fn id(&self) -> &str {
    &self.0
  }
}

#[derive(Debug, Clone)]
pub struct SessionType {
    pub path: PathBuf,
    pub session: String,
    pub episode: String,
    pub file: String
  }

impl TryFrom<EntryType> for SessionType {
  type Error = ();

  fn try_from(value: EntryType) -> Result<Self, Self::Error> {
    match value {
      EntryType::Session { path, session, episode, file } => {
        Ok(
          SessionType {
            path,
            session,
            episode,
            file
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
}

impl TryFrom<EntryType> for EncodeType {
    type Error = ();

    fn try_from(value: EntryType) -> Result<Self, Self::Error> {
      match value {
        EntryType::Encode { path, season } => {
          Ok(
            EncodeType {
              path,
              season
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
      session: session.to_owned(),
      episode: episode.to_owned(),
      file: file.to_owned()
    }
  }

  pub fn new_encodes<P: AsRef<Path>>(path: P, season: &str) -> Self {
    EntryType::Encode {
      path: path.as_ref().to_owned(),
      season: season.to_owned()
    }
  }
}
