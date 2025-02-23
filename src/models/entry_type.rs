use std::path::PathBuf;

use super::SessionId;
use super::MovieName;

// TODO: Extend EntryType to have one rename for tv series and one for movies
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

    // TODO: What does this mean in terms of a movie? Is it the name? Choose a better name for this field
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
