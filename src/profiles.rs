use serde_json::Value;
use walkdir::WalkDir;
use dirs::home_dir;
use std::fmt;

use crate::error::DeoError;

#[derive(Debug, Clone)]
pub struct ProfileConfigItem {
  full: String,
  display_name: String,
  preset_name: String
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

  pub fn display_name(&self) -> &str {
    self.display_name.as_str()
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

pub fn read_profile_config() -> Result<ProfileConfig, DeoError> {

  home_dir()
    .ok_or_else(|| DeoError::CouldNotFindHomeDir)
    .and_then(|hd| {
      let profiles_path = hd.join(".deo").join("profiles");
      if !(profiles_path.exists() && profiles_path.is_dir()) {
        Err(DeoError::ProfilesDirDoesNotExist(profiles_path.to_string_lossy().to_string()))
      } else {
        let profile_config_items: Result<Vec<ProfileConfigItem>, DeoError> =
          WalkDir::new(profiles_path.clone())
            .into_iter()
            .filter_map(|de| de.ok())
            .filter_map(|de| {
              if de.file_type().is_file() && de.path().extension().filter(|ext| &ext.to_string_lossy() == "json").is_some() {
                let result: Result<ProfileConfigItem, DeoError> =
                  std::fs::read_to_string(de.path())
                    .map_err(|e| DeoError::CouldNotReadProfile(de.path().to_string_lossy().to_string(), e.to_string() ))
                    .and_then(|profile_json| {
                        serde_json::from_str(&profile_json)
                          .map_err(|e| DeoError::CouldNotDecodeProfile(de.path().to_string_lossy().to_string(), e.to_string()) )
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
                                _ => Err(DeoError::ProfilePresetNameIsNotString(de.path().to_string_lossy().to_string())),
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
              Err(DeoError::NoProfilesFound(profiles_path.to_string_lossy().to_string()))
            } else {
              Ok(ProfileConfig(profile_items))
            }
          })
        }
    })



}
