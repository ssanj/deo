use std::fmt;
use std::path::PathBuf;
use console::style;

use crate::profiles::ProfileConfigItem;
use crate::models::SessionId;
use crate::models::SessionToEncodeDir;

pub struct UserSelection {
  session_id: SessionId,
  session_to_encode_dir: SessionToEncodeDir,
  profile: ProfileConfigItem
}

impl UserSelection {
  pub fn new(session_id: SessionId, session_to_encode_dir: SessionToEncodeDir, profile: ProfileConfigItem) -> Self {
    Self {
      session_id,
      session_to_encode_dir,
      profile
    }
  }

  pub fn session_to_encode_dir(&self) -> SessionToEncodeDir {
    self.session_to_encode_dir.clone()
  }


  pub fn encode_dir_path(&self) -> PathBuf {
    self.session_to_encode_dir.encode_dir_path()
  }

  pub fn profile(&self) -> &ProfileConfigItem {
    &self.profile
  }
}

impl fmt::Display for UserSelection {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let UserSelection { session_id, session_to_encode_dir, profile } = self;
    let location = &session_to_encode_dir.location();
    write!(f, "Copy {} -> {} with {}", style(session_id).yellow(), style(location).underlined(), style(profile).blue())
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
