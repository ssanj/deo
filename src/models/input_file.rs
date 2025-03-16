use crate::models::MovieRenameFile;
use crate::models::TVSeriesRenameFile;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct InputFile {
  pub mkv_file: String,
  pub mp4_file: String,
  pub mkv_path: PathBuf,
}

impl From<TVSeriesRenameFile> for InputFile {
  fn from(tv_rename: TVSeriesRenameFile) -> Self {
      Self {
        mkv_file: tv_rename.mkv_file,
        mp4_file: tv_rename.mp4_file,
        mkv_path: tv_rename.path,
      }
  }
}

impl From<MovieRenameFile> for InputFile {
  fn from(movie_rename: MovieRenameFile) -> Self {
      Self {
        mkv_file: movie_rename.mkv_file,
        mp4_file: movie_rename.mp4_file,
        mkv_path: movie_rename.path,
      }
  }
}
