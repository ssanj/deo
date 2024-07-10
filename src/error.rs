#[derive(Debug)]
pub enum DeoError {
  CouldNotReadProfile(String, String),
  CouldNotDecodeProfile(String, String),
  ProfilePresetNameIsNotString(String),
  ProfilesDirDoesNotExist(String),
  CouldNotFindHomeDir,
  NoProfilesFound(String),
}
