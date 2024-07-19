use std::{fmt, path::Path};

#[derive(Debug)]
pub enum DeoError {
  CouldNotReadProfile(String, String),
  CouldNotDecodeProfile(String, String),
  ProfilePresetNameIsNotString(String),
  ProfilesDirDoesNotExist(String),
  CouldNotFindHomeDir,
  NoProfilesFound(String),
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
