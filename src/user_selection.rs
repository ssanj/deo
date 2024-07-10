use std::fmt;
use crate::entry_type::{EncodeDir, RenameFile, SessionId, SessionToEncodeDir};

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

#[derive(Debug, Clone)]
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


pub struct UserSelection {
  session_id: SessionId,
  session_to_encode_dir: SessionToEncodeDir,
  profile: Profile
}

impl <'a> UserSelection {
  pub fn new(session_id: SessionId, session_to_encode_dir: SessionToEncodeDir, profile: Profile) -> Self {
    Self {
      session_id,
      session_to_encode_dir,
      profile
    }
  }

  pub fn rename_files(&self) -> &[RenameFile] {
    &self.session_to_encode_dir.session().files()
  }

  pub fn encode_dir(&self) -> &EncodeDir {
    &self.session_to_encode_dir.encode_dir()
  }

  pub fn profile(&self) -> &Profile {
    &self.profile
  }
}

impl fmt::Display for UserSelection {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let UserSelection { session_id, session_to_encode_dir, profile } = self;
      let season = &session_to_encode_dir.encode_dir().season;
      write!(f, "Copy {} -> {} with {} profile", session_id, season, profile)
  }
}

pub enum ContinueType {
  EncodeSelection,
  Cancel,
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
