use std::path::Path;
use crate::models::SessionId;
use crate::models::Session;
use crate::models::MovieName;
use crate::models::EntryType;
use crate::models::EncodeDirType;
use crate::models::TVSeriesEncodeDir;
use crate::models::MovieEncodeDir;


impl EntryType {
  pub fn new_tv_series_rename<P: AsRef<Path>>(path: P, session: &str, episode: &str, file: &str) -> Self {
    EntryType::TVSeriesRename {
      path: path.as_ref().to_owned(),
      session: SessionId::new(session),
      episode: episode.to_owned(),
      file: file.to_owned()
    }
  }

  pub fn new_movie_rename<P: AsRef<Path>>(path: P, session: &str, file: &str) -> Self {
    EntryType::MovieRename {
      path: path.as_ref().to_owned(),
      session: SessionId::new(session),
      file: file.to_owned()
    }
  }

  pub fn new_tv_series_encodes<P: AsRef<Path>>(path: P, season: &str, session: &str) -> Self {
    EntryType::TVSeriesEncode {
      path: path.as_ref().to_owned(),
      season: season.to_owned(),
      session: SessionId::new(session)
    }
  }

  pub fn new_movie_encodes<P: AsRef<Path>>(path: P, movie_name: &str, session: &str) -> Self {
    EntryType::MovieEncode {
      path: path.as_ref().to_owned(),
      movie_name: MovieName::new(movie_name),
      session: SessionId::new(session)
    }
  }

  pub(crate) fn could_not_match_defined_encode_dir(encode_file_contents: &str) -> EntryType {
    EntryType::InvalidEncodeDirPath {
      defined_path: encode_file_contents.to_owned()
    }
  }

  pub(crate) fn unknown_file_type<P: AsRef<Path>>(path: P) -> EntryType {
    EntryType::UnknownFileType {
      path: path.as_ref().to_owned()
    }
  }
}


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
