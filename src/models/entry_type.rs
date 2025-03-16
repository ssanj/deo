use std::path::Path;
use std::path::PathBuf;

use super::SessionId;
use super::MovieName;

// This class models something that can be encoded later.
#[derive(Debug, Clone)]
pub enum EntryType {
  TVSeriesRename {

    /// Full path to mkv file
    path: PathBuf,

    /// Session id of file
    session: SessionId,

    /// Episode
    episode: String,

    /// file name and extension
    file: String
  },

  MovieRename {

    /// Full path to mkv file
    path: PathBuf,

    /// Session id of file
    session: SessionId,

    /// file name and extension
    file: String
  },

  TVSeriesEncode {

    /// Session id of files that map to this encode directory
    session: SessionId,

    /// Full path to encode directory
    path: PathBuf,

    /// Season
    season: String,
  },

  MovieEncode {

    /// Session id of files that map to this encode directory
    session: SessionId,

    /// Full path to encode directory
    path: PathBuf,

    /// Name of the movie
    movie_name: MovieName
  },

  UnknownFileType {
    path: PathBuf
  },


  InvalidEncodeDirPath {
    defined_path: String
  }
}

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
