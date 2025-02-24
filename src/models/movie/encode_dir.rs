use super::SessionId;
use super::MovieSession;
use super::MovieEncodeDir;

pub struct MovieToEncodeDir {
  session_id: SessionId,
  session: MovieSession,
  encode_dir: MovieEncodeDir
}
