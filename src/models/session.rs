use super::SessionId;
use super::RenameTypes;
use super::MKVTypeAware;

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
    sorted_files.sort_by_key(|a| a.mkv_file());
    sorted_files
  }

  pub fn id(&self) -> SessionId {
    self.session_id.clone()
  }
}
