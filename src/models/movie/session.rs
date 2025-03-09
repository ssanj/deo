use super::MovieRenameFile;
use super::SessionId;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MovieSession {
  session_id: SessionId,
  files: Vec<MovieRenameFile>,
}

impl MovieSession {
  pub fn new(session_id: SessionId, files: Vec<MovieRenameFile>) -> Self {
    Self {
      session_id,
      files
    }
  }

  pub fn files(&self) -> Vec<MovieRenameFile> {
    self.files.clone()
  }
}
