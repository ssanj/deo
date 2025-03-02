use crate::models::SessionTypeAware;

use super::SessionId;
use super::MovieSession;
use super::MovieEncodeDir;

#[derive(Debug, Clone)]
pub struct MovieToEncodeDir {
  session_id: SessionId,
  session: MovieSession,
  encode_dir: MovieEncodeDir
}

impl MovieToEncodeDir {
  pub fn new(session_id: SessionId, session: MovieSession, encode_dir: MovieEncodeDir) -> Self {
      Self {
        session_id,
        session,
        encode_dir
      }
  }

  pub fn session(&self) -> MovieSession {
    self.session.clone()
  }

  pub fn encode_dir(&self) -> MovieEncodeDir {
    self.encode_dir.clone()
  }
}


impl SessionTypeAware for MovieToEncodeDir {
    fn session_id(&self) -> SessionId {
      self.session_id.clone()
    }

}
