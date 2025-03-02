use std::collections::HashMap;

use super::EncodeDirPathAware;
use super::EncodeDirType;
use super::InputFile;
use super::LocationAware;
use super::MKVTypeAware;
use super::RenameTypes;
use super::SessionId;
use super::Session;
use super::TVSeriesEncodeDir;
use super::MovieEncodeDir;
use super::tv_series::TVSeriesToEncodeDir;
use super::tv_series::TVSeriesSession;
use super::movie::MovieToEncodeDir;
use super::movie::MovieSession;
use super::SessionTypeAware;


#[derive(Debug, Clone)]
pub enum SessionToEncodeDir {
  TVSeriesMapping(TVSeriesToEncodeDir),
  MovieMapping(MovieToEncodeDir)
}

impl SessionToEncodeDir {
  pub fn new_tv_series_encode_dir(session_id: SessionId, session: TVSeriesSession, encode_dir: TVSeriesEncodeDir) -> Self {
    SessionToEncodeDir::TVSeriesMapping(
      TVSeriesToEncodeDir::new(
        session_id,
        session,
        encode_dir
      )
    )
  }

  pub fn new_movie_encode_dir(session_id: SessionId, session: MovieSession, encode_dir: MovieEncodeDir) -> Self {
    SessionToEncodeDir::MovieMapping(
      MovieToEncodeDir::new(
        session_id,
        session,
        encode_dir
      )
    )
  }


  pub fn from_tvseries_elements(tv_series_session: HashMap<SessionId, TVSeriesSession>, tv_series_encode_dir: HashMap<SessionId, TVSeriesEncodeDir>) -> Vec<SessionToEncodeDir> {

    tv_series_session
      .iter()
      .filter_map(|(session_id, session_dir)| {
        tv_series_encode_dir
          .get(session_id)
          .map(|encode_dir| {
            Self::new_tv_series_encode_dir(session_id.clone(), session_dir.clone(), encode_dir.clone())
          })
      })
      .collect()
  }

  pub fn from_movie_elements(movie_session: HashMap<SessionId, MovieSession>, movie_encode_dir: HashMap<SessionId, MovieEncodeDir>) -> Vec<SessionToEncodeDir> {

    movie_session
      .iter()
      .filter_map(|(session_id, session_dir)| {
        movie_encode_dir
          .get(session_id)
          .map(|encode_dir| {
            Self::new_movie_encode_dir(session_id.clone(), session_dir.clone(), encode_dir.clone())
          })
      })
      .collect()
  }

  pub fn rename_files(&self) -> Vec<InputFile> {
    let result = match self.clone() {
        SessionToEncodeDir::TVSeriesMapping(tvseries_to_encode_dir) =>
          tvseries_to_encode_dir
            .session()
            .files()
            .into_iter()
            .map(|tv| tv.into())
            .collect(),
        SessionToEncodeDir::MovieMapping(movie_to_encode_dir) =>
          movie_to_encode_dir
            .session()
            .files()
            .into_iter()
            .map(|movie| movie.into())
            .collect()
    };

    result
  }

  pub fn file_count(&self) -> u64 {
    let result = match self.clone() {
        SessionToEncodeDir::TVSeriesMapping(tvseries_to_encode_dir) =>
          tvseries_to_encode_dir
            .session()
            .files()
            .len() as u64,
        SessionToEncodeDir::MovieMapping(movie_to_encode_dir) =>
          movie_to_encode_dir
            .session()
            .files()
            .len() as u64
    };

    result
  }
}

impl SessionTypeAware for SessionToEncodeDir {
  fn session_id(&self) -> SessionId {
    match self {
        SessionToEncodeDir::TVSeriesMapping(tvseries_to_encode_dir) => tvseries_to_encode_dir.session_id(),
        SessionToEncodeDir::MovieMapping(movie_to_encode_dir) => movie_to_encode_dir.session_id(),
    }
  }
}

impl LocationAware for SessionToEncodeDir {
    fn location(&self) -> String {
      match self {
        SessionToEncodeDir::TVSeriesMapping(tvseries_to_encode_dir) => tvseries_to_encode_dir.encode_dir().season,
        SessionToEncodeDir::MovieMapping(movie_to_encode_dir) => movie_to_encode_dir.encode_dir().movie_name.name(),
      }
    }
}

impl EncodeDirPathAware for SessionToEncodeDir {
    fn encode_dir_path(&self) -> std::path::PathBuf {
      match self {
        SessionToEncodeDir::TVSeriesMapping(tvseries_to_encode_dir) => tvseries_to_encode_dir.encode_dir().path,
        SessionToEncodeDir::MovieMapping(movie_to_encode_dir) => movie_to_encode_dir.encode_dir().path,
      }
    }
}
