use super::EncodeDirType;
use super::SessionId;
use super::Session;
use super::TVSeriesEncodeDir;
use super::MovieEncodeDir;


#[derive(Debug)]
pub struct SessionToEncodeDir {
  session_id: SessionId,
  session: Session,
  encode_dir: EncodeDirType
}

impl SessionToEncodeDir {
  pub fn new_tv_series_encode_dir(session_id: SessionId, session: Session, encode_dir: TVSeriesEncodeDir) -> Self {
    Self {
      session_id,
      session,
      encode_dir: EncodeDirType::TVSeries(encode_dir)
    }
  }

  pub fn new_movie_encode_dir(session_id: SessionId, session: Session, encode_dir: MovieEncodeDir) -> Self {
    Self {
      session_id,
      session,
      encode_dir: EncodeDirType::Movie(encode_dir)
    }
  }
}

impl SessionToEncodeDir {
  pub fn session_id(&self) -> &SessionId {
    &self.session_id
  }

  pub fn session(&self) -> &Session {
    &self.session
  }

  pub fn encode_dir(&self) -> &EncodeDirType {
    &self.encode_dir
  }
}
