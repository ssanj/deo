use crate::models::MKVTypeAware;

use super::SessionId;
use super::TVSeriesRenameFile;

#[derive(Debug, Clone)]
pub struct TVSeriesSession {
  session_id: SessionId,
  files: Vec<TVSeriesRenameFile>,
}


impl TVSeriesSession {
  pub fn new(session_id: SessionId, files: Vec<TVSeriesRenameFile>) -> Self {
    Self {
      session_id,
      files
    }
  }

  pub fn files(&self) -> Vec<TVSeriesRenameFile> {
    self.files.clone()
  }
}
