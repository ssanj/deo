use super::SessionId;
use super::TVSeriesRenameFile;

#[derive(Debug, Clone, Eq, PartialEq)]
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

  pub fn files_sorted(&self) -> Vec<TVSeriesRenameFile> {
    let mut sorted = self.files.clone();
    sorted.sort_by(|a, b| a.episode.cmp(&b.episode));
    sorted
  }

  pub fn session_id(&self) -> SessionId {
    self.session_id.clone()
  }
}
