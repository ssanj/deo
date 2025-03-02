use std::path::PathBuf;

use crate::models::MovieRenameFile;
use crate::models::TVSeriesRenameFile;
use crate::profiles::ProfileConfigItem;
use crate::user_selection::UserSelection;

#[derive(Debug, Clone)]
pub struct HandbrakeInputFile {
  pub mkv_file: String,
  pub mp4_file: String,
  pub mkv_path: PathBuf,
}

impl From<TVSeriesRenameFile> for HandbrakeInputFile {
  fn from(tv_rename: TVSeriesRenameFile) -> Self {
      Self {
        mkv_file: tv_rename.mkv_file,
        mp4_file: tv_rename.mp4_file,
        mkv_path: tv_rename.path,
      }
  }
}

impl From<MovieRenameFile> for HandbrakeInputFile {
  fn from(movie_rename: MovieRenameFile) -> Self {
      Self {
        mkv_file: movie_rename.mkv_file,
        mp4_file: movie_rename.mp4_file,
        mkv_path: movie_rename.path,
      }
  }
}

#[derive(Debug, Clone)]
pub struct HandbrakeInfo {
  pub encode_dir_path: PathBuf,
  pub profile: ProfileConfigItem,
  pub input_files: Vec<HandbrakeInputFile>
}


impl From<UserSelection> for HandbrakeInfo {
  fn from(user_selection: UserSelection) -> Self {
      let input_files: Vec<HandbrakeInputFile> =
        match user_selection.session_to_encode_dir() {
            crate::models::SessionToEncodeDir::TVSeriesMapping(tvseries_to_encode_dir) => {
              tvseries_to_encode_dir
                .session()
                .files()
                .into_iter()
                .map(|tv| tv.into())
                .collect()
            },
            crate::models::SessionToEncodeDir::MovieMapping(movie_to_encode_dir) => {
              movie_to_encode_dir
                .session()
                .files()
                .into_iter()
                .map(|movie| movie.into())
                .collect()
            },
        };

      let encode_dir_path = user_selection.encode_dir_path();
      let profile = user_selection.profile().clone();


      Self {
        input_files,
        profile,
        encode_dir_path,
      }
  }
}
