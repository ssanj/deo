use super::SessionId;
use super::TVSeriesSession;
use super::TVSeriesEncodeDir;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TVSeriesToEncodeDir {
  session_id: SessionId,
  session: TVSeriesSession,
  encode_dir: TVSeriesEncodeDir,
}

impl TVSeriesToEncodeDir {
  pub fn new(session_id: SessionId, session: TVSeriesSession, encode_dir: TVSeriesEncodeDir) -> Self {
    Self {
      session_id,
      session,
      encode_dir
    }
  }

  pub fn session(&self) -> TVSeriesSession {
    self.session.clone()
  }

  #[cfg(test)]
  pub fn sorted_session(&self) -> TVSeriesSession {
    TVSeriesSession::new(self.session_id.clone(), self.session.clone().files_sorted())
  }

  pub fn encode_dir(&self) -> TVSeriesEncodeDir {
    self.encode_dir.clone()
  }

  pub fn session_id(&self) -> SessionId {
    self.session_id.clone()
  }
}

