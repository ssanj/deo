use serde_json::Value;
use walkdir::WalkDir;
use dirs::home_dir;
use std::fmt;

use crate::error::{DeoProfileError, DirName, FileName};

#[derive(Debug, Clone)]
pub struct ProfileConfigItem {
  full: String,
  display_name: String,
  preset_name: String
}

#[derive(Debug)]
pub enum ProfileSelection {
  Select(ProfileConfigItem),
  Skip
}

impl fmt::Display for ProfileSelection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let item = match self {
        ProfileSelection::Select(pci) => pci.to_string(),
        ProfileSelection::Skip => "Skip".to_owned(),
      };

      write!(f, "{}", item)
    }
}

#[derive(Debug, Clone)]
pub struct ProfileConfig(Vec<ProfileConfigItem>);

impl ProfileConfig {
  pub fn items(&self) -> &[ProfileConfigItem] {
    &self.0
  }
}

impl ProfileConfigItem {
  pub fn full_path(&self) -> &str {
    self.full.as_str()
  }

  pub fn preset_name(&self) -> &str {
    self.preset_name.as_str()
  }
}

impl fmt::Display for ProfileConfigItem {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", self.display_name)
  }
}

pub fn read_profile_config() -> Result<ProfileConfig, DeoProfileError> {

  home_dir()
    .ok_or(DeoProfileError::CouldNotFindHomeDir)
    .and_then(|hd| {
      let profiles_path = hd.join(".deo").join("profiles");
      if !(profiles_path.exists() && profiles_path.is_dir()) {
        Err(DeoProfileError::ProfilesDirDoesNotExist(DirName::new(profiles_path)))
      } else {
        let profile_config_items: Result<Vec<ProfileConfigItem>, DeoProfileError> =
          WalkDir::new(profiles_path.clone())
            .into_iter()
            .filter_map(|de| de.ok())
            .filter_map(|de| {
              if de.file_type().is_file() && de.path().extension().filter(|ext| &ext.to_string_lossy() == "json").is_some() {
                let result: Result<ProfileConfigItem, DeoProfileError> =
                  std::fs::read_to_string(de.path())
                    .map_err(|e| DeoProfileError::CouldNotReadProfile(FileName::new(de.path()), e.to_string() ))
                    .and_then(|profile_json| {
                        serde_json::from_str(&profile_json)
                          .map_err(|e| DeoProfileError::CouldNotDecodeProfile(FileName::new(de.path()), e.to_string()) )
                          .and_then(|json: Value| {
                            let preset_name_value = &json["PresetList"][0]["PresetName"];

                            match preset_name_value {
                                Value::String(preset_name) => {
                                  let display_name = de.path().file_stem().map_or_else(|| "<Unknown>".to_owned(), |filestem| filestem.to_string_lossy().to_string());
                                  let full = de.path().to_string_lossy().to_string();
                                  Ok(
                                    ProfileConfigItem {
                                      full,
                                      display_name,
                                      preset_name: preset_name.to_owned(),
                                    }
                                  )
                                },
                                value => Err(DeoProfileError::ProfilePresetNameIsNotString(FileName::new(de.path()), value.to_string())),
                            }
                          })
                  });

                Some(result)
              } else {
                None
              }
            })
            .collect();


        profile_config_items
          .and_then(|profile_items| {
            if profile_items.is_empty() {
              Err(DeoProfileError::NoProfilesFound(DirName::new(profiles_path)))
            } else {
              Ok(ProfileConfig(profile_items))
            }
          })
        }
    })
}
