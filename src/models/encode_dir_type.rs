use std::path::PathBuf;

use super::SessionId;
use super::MovieName;
use super::EntryType;
use super::SessionTypeAware;

#[derive(Debug, Clone)]
pub enum EncodeDirType {
  TVSeries(TVSeriesEncodeDir),
  Movie(MovieEncodeDir),
}

pub trait EncodeDirPathAware {
  fn path(&self) -> PathBuf;
}

pub trait LocationAware {
  fn location(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct TVSeriesEncodeDir {
  pub path: PathBuf,
  pub season: String,
  pub session_id: SessionId,
}

#[derive(Debug, Clone)]
pub struct MovieEncodeDir {
  pub path: PathBuf,
  pub session_id: SessionId,
  pub movie_name: MovieName
}

impl TryFrom<EntryType> for EncodeDirType {
    type Error = ();

    fn try_from(value: EntryType) -> Result<Self, Self::Error> {
      match value {
        EntryType::TVSeriesEncode { path, season, session } => {
          let session_id = session;
          Ok(
            EncodeDirType::TVSeries(
              TVSeriesEncodeDir {
                path,
                season,
                session_id
              }
            )
          )
        },
        EntryType::MovieEncode { path, session, movie_name } => {
          let session_id = session;
          Ok(
            EncodeDirType::Movie(
              MovieEncodeDir {
                path,
                session_id,
                movie_name
              }
            )
          )
        },
        _ => {
          Err(())
        },
    }
  }
}

impl SessionTypeAware for EncodeDirType {
    fn session_id(&self) -> SessionId {
      match self {
        EncodeDirType::TVSeries(tvseries_encode_dir) => tvseries_encode_dir.session_id.clone(),
        EncodeDirType::Movie(movie_encode_dir) => movie_encode_dir.session_id.clone(),
      }
    }
}

impl EncodeDirPathAware for EncodeDirType {
    fn path(&self) -> PathBuf {
        match self {
            EncodeDirType::TVSeries(tvseries_encode_dir) => tvseries_encode_dir.path.clone(),
            EncodeDirType::Movie(movie_encode_dir) => movie_encode_dir.path.clone(),
        }
    }
}


impl LocationAware for EncodeDirType {
    fn location(&self) -> String {
      match self {
        EncodeDirType::TVSeries(tvseries_encode_dir) => tvseries_encode_dir.season.clone(),
        EncodeDirType::Movie(movie_encode_dir) => movie_encode_dir.movie_name.to_string(),
      }
    }
}
