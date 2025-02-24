use super::SessionId;
use super::TVSeriesSession;
use super::TVSeriesEncodeDir;

pub struct TVSeriesToEncodeDir {
  session_id: SessionId,
  session: TVSeriesSession,
  encode_dir: TVSeriesEncodeDir,
}
