use crate::models::session_id;

use super::MovieRenameFile;
use super::SessionId;

#[derive(Debug, Clone)]
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
