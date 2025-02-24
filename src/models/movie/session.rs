use super::MovieRenameFile;
use super::SessionId;

pub struct MovieSession {
  session_id: SessionId,
  files: Vec<MovieRenameFile>,
}
