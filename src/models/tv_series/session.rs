use super::SessionId;
use super::TVSeriesRenameFile;

pub struct TVSeriesSession {
  session_id: SessionId,
  files: Vec<TVSeriesRenameFile>,
}
