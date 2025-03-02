use std::collections::HashMap;
use std::path::PathBuf;

use super::SessionId;
use super::MovieName;
use super::EntryType;

#[derive(Debug, Clone)]
pub enum EncodeDirType {
  TVSeries(TVSeriesEncodeDir),
  Movie(MovieEncodeDir),
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

/// Convert from a collection of EncodeDirType into a pair of HashMaps for each EncodeDirType:
/// `HashMap<SessionId, Vec<TVSeriesEncodeDir>>` and
/// `HashMap<SessionId, Vec<MovieEncodeDir>>`
impl FromIterator<EncodeDirType> for (HashMap<SessionId, TVSeriesEncodeDir>, HashMap<SessionId, MovieEncodeDir>) {

  fn from_iter<T: IntoIterator<Item = EncodeDirType>>(renames: T) -> Self {
    let mut hash: HashMap<SessionId, (Option<TVSeriesEncodeDir>, Option<MovieEncodeDir>)> = HashMap::new();
      for rename in renames {
        match rename {
            EncodeDirType::TVSeries(tvseries_encode_dir) => {
              hash.insert(tvseries_encode_dir.clone().session_id, (Some(tvseries_encode_dir), None));
            },
            EncodeDirType::Movie(movie_encode_dir) => {
              hash.insert(movie_encode_dir.clone().session_id, (None, Some(movie_encode_dir)));
            },
        }
      }

      let tv_encodes_hash: HashMap<SessionId, TVSeriesEncodeDir> =
        hash
          .clone()
          .into_iter()
          .filter_map(|(k, (p_tv, _))| {
            p_tv.map(|tv| (k.clone(), tv))
          })
          .collect();

      let movie_encodes_hash: HashMap<SessionId, MovieEncodeDir> =
        hash
          .into_iter()
          .filter_map(|(k, (_, p_movie))| {
            p_movie.map(|movie| (k.clone(), movie))
          })
          .collect();

      (tv_encodes_hash, movie_encodes_hash)
    }
}
