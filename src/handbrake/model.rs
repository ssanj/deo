use std::path::PathBuf;

use crate::models::InputFile;
use crate::profiles::ProfileConfigItem;
use crate::user_selection::UserSelection;

#[derive(Debug, Clone)]
pub struct HandbrakeInfo {
  pub encode_dir_path: PathBuf,
  pub profile: ProfileConfigItem,
  pub input_files: Vec<InputFile>
}


impl From<UserSelection> for HandbrakeInfo {
  fn from(user_selection: UserSelection) -> Self {
      let input_files: Vec<InputFile> = user_selection.session_to_encode_dir().rename_files();

      let encode_dir_path = user_selection.encode_dir_path();
      let profile = user_selection.profile().clone();


      Self {
        input_files,
        profile,
        encode_dir_path,
      }
  }
}
