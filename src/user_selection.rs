use std::fmt;
use crate::entry_type::{EncodeType, SessionType};

pub enum EncodeOption {
  Encode(EncodeType),
  Skip,
  Done
}

impl fmt::Display for EncodeOption {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let option = match self {
      EncodeOption::Encode(EncodeType { season, .. }) => season,
      EncodeOption::Skip => "Skip",
      EncodeOption::Done => "Done",
    };

    write!(f, "{}", option)
  }
}

pub enum Profile {
  Dvd,
  Bluray
}

impl fmt::Display for Profile {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let option = match self {
      Profile::Dvd => "DVD",
      Profile::Bluray => "Bluray",
    };

    write!(f, "{}", option)
  }
}

pub struct UserSelection<'a>(Vec<SessionType>, &'a EncodeType, &'a Profile);

impl <'a> UserSelection<'a> {
  pub fn new(session_types: Vec<SessionType>, encode_type: &'a EncodeType, profile: &'a Profile) -> Self {
    Self(session_types, encode_type, profile)
  }
}

impl fmt::Display for UserSelection<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let UserSelection(session_type, encode_type, profile) = self;
      write!(f, "Copy {} -> {} with {} profile", session_type.first().unwrap().session, encode_type.season, profile)
  }
}

pub enum ContinueType {
  EncodeSelection,
  Cancel
}

impl fmt::Display for ContinueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let option = match self {
        ContinueType::EncodeSelection => "Encode selection",
        ContinueType::Cancel => "Cancel",
      };

      write!(f, "{}", option)
    }
}
