use std::fmt;
use crate::entry_type::{EncodeDir, SessionId, SessionType};

pub enum EncodeOption {
  Encode(EncodeDir),
  Skip,
  Done
}

impl fmt::Display for EncodeOption {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let option = match self {
      EncodeOption::Encode(EncodeDir { season, .. }) => season,
      EncodeOption::Skip => "Skip",
      EncodeOption::Done => "Done",
    };

    write!(f, "{}", option)
  }
}

pub enum Profile {
  Dvd,
  Bluray
}

impl fmt::Display for Profile {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let option = match self {
      Profile::Dvd => "DVD",
      Profile::Bluray => "Bluray",
    };

    write!(f, "{}", option)
  }
}

/// A Session has a SessionId and a list of files (SessionType)
pub struct Session {
  session_id: SessionId,
  files: Vec<SessionType>,
}

impl Session {

  pub fn new(session_id: SessionId, files: Vec<SessionType>) -> Self {
    Self {
      session_id,
      files
    }
  }

  pub fn first(&self) -> Option<SessionType> {
    self.files.first().cloned()
  }
}

pub struct UserSelection<'a> {
  session: Session,
  encode_dir: &'a EncodeDir,
  profile: &'a Profile
}

impl <'a> UserSelection<'a> {
  pub fn new(session_id: SessionId, session_types: Vec<SessionType>, encode_dir: &'a EncodeDir, profile: &'a Profile) -> Self {
    let session = Session::new(session_id, session_types);
    Self {
      session,
      encode_dir,
      profile
    }
  }

  pub fn session_types(&self) -> &[SessionType] {
    &self.session.files
  }

  pub fn encode_type(&self) -> &EncodeDir {
    &self.encode_dir
  }

  pub fn profile(&self) -> &Profile {
    &self.profile
  }
}

impl fmt::Display for UserSelection<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let UserSelection { session, encode_dir, profile } = self;
      let session_id = &session.session_id;
      let season = &encode_dir.season;
      write!(f, "Copy {} -> {} with {} profile", session_id, season, profile)
  }
}

pub enum ContinueType {
  EncodeSelection,
  Cancel
}

impl fmt::Display for ContinueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let option = match self {
        ContinueType::EncodeSelection => "Encode selection",
        ContinueType::Cancel => "Cancel",
      };

      write!(f, "{}", option)
    }
}
