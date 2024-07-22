use std::{fmt, path::Path};

#[derive(Debug)]
pub enum DeoProfileError {
  CouldNotReadProfile(FileName, String),
  CouldNotDecodeProfile(FileName, String),
  ProfilePresetNameIsNotString(FileName, String),
  ProfilesDirDoesNotExist(DirName),
  CouldNotFindHomeDir,
  NoProfilesFound(DirName),
}

impl fmt::Display for DeoProfileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let item = match self {
        DeoProfileError::CouldNotReadProfile(profile, error) => format!("Could not read profile file: {profile}, due to: {error}"),
        DeoProfileError::CouldNotDecodeProfile(profile, error) => format!("Could not decode profile file: {profile}, due to: {error}"),
        DeoProfileError::ProfilePresetNameIsNotString(profile, error) => format!("Profile: {profile} has an invalid preset value: {error}"),
        DeoProfileError::ProfilesDirDoesNotExist(error) => format!("Profile directory does not exist: {error}"),
        DeoProfileError::CouldNotFindHomeDir => "Could not find home directory".to_owned(),
        DeoProfileError::NoProfilesFound(error) => format!("Could not find any profiles at: {error}"),
      };

      write!(f, "{}", item)
    }
}

#[derive(Debug)]
pub struct FileName(String);

impl FileName {
  pub fn new<P: AsRef<Path>>(path: P) -> Self {
    Self(path.as_ref().to_string_lossy().to_string())
  }
}

impl fmt::Display for FileName {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", self.0)
  }
}

#[derive(Debug)]
pub struct DirName(String);

impl DirName {
  pub fn new<P: AsRef<Path>>(path: P) -> Self {
    Self(path.as_ref().to_string_lossy().to_string())
  }
}

impl fmt::Display for DirName {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", self.0)
  }
}


#[derive(Debug)]
pub struct LogFile(String);

impl LogFile {
  pub fn new<P: AsRef<Path>>(path: P) -> Self {
    Self(path.as_ref().to_string_lossy().to_string())
  }
}

#[derive(Debug)]
pub struct HandbrakeCommand(String);

impl HandbrakeCommand {
  pub fn new(cmd: String) -> Self {
    HandbrakeCommand(cmd)
  }
}

#[derive(Debug)]
pub enum DeoEncodingError {
  CouldNotRemoveLogFile(LogFile, String),
  CouldNotOpenLogFile(LogFile, String),
  FailedToSpawnHandbrake(HandbrakeCommand, String),
}

impl fmt::Display for DeoEncodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let item = match self {
          DeoEncodingError::CouldNotRemoveLogFile(log_file, error) => format!("Could not remove log file: {} due to: {}", log_file.0, error),
          DeoEncodingError::CouldNotOpenLogFile(log_file, error) => format!("Could not open log file: {} due to: {}", log_file.0, error),
          DeoEncodingError::FailedToSpawnHandbrake(cmd, error) => format!("Could not spawn handbrake command: '{}' due to: {}", cmd.0, error),
        };

        write!(f, "{}", item)
    }
}
