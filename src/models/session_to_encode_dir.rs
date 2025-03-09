use std::collections::HashMap;

use super::InputFile;
use super::SessionId;
use super::TVSeriesEncodeDir;
use super::MovieEncodeDir;
use super::tv_series::TVSeriesToEncodeDir;
use super::tv_series::TVSeriesSession;
use super::movie::MovieToEncodeDir;
use super::movie::MovieSession;

#[derive(Debug, Clone, Eq, PartialEq)]
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


  pub fn from_tvseries_elements(tv_series_session: &HashMap<SessionId, TVSeriesSession>, tv_series_encode_dir: &HashMap<SessionId, TVSeriesEncodeDir>) -> Vec<SessionToEncodeDir> {

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

  pub fn from_movie_elements(movie_session: &HashMap<SessionId, MovieSession>, movie_encode_dir: &HashMap<SessionId, MovieEncodeDir>) -> Vec<SessionToEncodeDir> {

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

  pub fn session_id(&self) -> SessionId {
    match self {
        SessionToEncodeDir::TVSeriesMapping(tvseries_to_encode_dir) => tvseries_to_encode_dir.session_id(),
        SessionToEncodeDir::MovieMapping(movie_to_encode_dir) => movie_to_encode_dir.session_id(),
    }
  }

  pub fn location(&self) -> String {
    match self {
      SessionToEncodeDir::TVSeriesMapping(tvseries_to_encode_dir) => tvseries_to_encode_dir.encode_dir().season,
      SessionToEncodeDir::MovieMapping(movie_to_encode_dir) => movie_to_encode_dir.encode_dir().movie_name.name(),
    }
  }

  pub fn encode_dir_path(&self) -> std::path::PathBuf {
    match self {
      SessionToEncodeDir::TVSeriesMapping(tvseries_to_encode_dir) => tvseries_to_encode_dir.encode_dir().path,
      SessionToEncodeDir::MovieMapping(movie_to_encode_dir) => movie_to_encode_dir.encode_dir().path,
    }
  }

  pub fn sorted_files(&self) -> Self {
    match self {
        SessionToEncodeDir::TVSeriesMapping(tvseries_to_encode_dir) => {
          SessionToEncodeDir::TVSeriesMapping(
            TVSeriesToEncodeDir::new(
              tvseries_to_encode_dir.session_id(),
              tvseries_to_encode_dir.sorted_session(),
              tvseries_to_encode_dir.encode_dir()
            )
          )
        },
        SessionToEncodeDir::MovieMapping(movie_to_encode_dir) => todo!(),
    }
  }
}
