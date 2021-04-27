use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::path_extension::PathExtension;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupConfiguration {
    #[serde(default)]
    pub links: Option<HashMap<String, LinkConfiguration>>,
    #[serde(default)]
    pub clean: Option<CleanConfiguration>,
    #[serde(default = "default_relative")]
    pub relative: Option<bool>,
    #[serde(default = "default_force")]
    pub force: Option<bool>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkConfiguration {
    pub target: String,
    #[serde(default = "default_target_relative")]
    pub relative: Option<bool>,
    #[serde(default = "default_force")]
    pub force: Option<bool>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetConfiguration {
    #[serde(default = "default_force")]
    pub force: Option<bool>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CleanConfiguration {
    #[serde(default)]
    pub targets: Option<HashMap<String, TargetConfiguration>>,
    #[serde(default)]
    pub force: Option<bool>,
}

fn default_relative() -> Option<bool> {
    return Some(true);
}

fn default_target_relative() -> Option<bool> {
    return None;
}

fn default_force() -> Option<bool> {
    return None;
}

pub fn detect_configuration_path<P: AsRef<Path>>(directory: P) -> Option<PathBuf> {
    let path = directory.as_ref().absolutize().ok()?;
    if path.exists() && path.is_dir() {
        let json_options_path = path.join("idot.json");
        if json_options_path.exists() && json_options_path.is_file() {
            return Some(json_options_path);
        }
        let toml_options_path = path.join("idot.toml");
        if toml_options_path.exists() && toml_options_path.is_file() {
            return Some(toml_options_path);
        }
        let yaml_options_path = path.join("idot.yaml");
        if yaml_options_path.exists() && yaml_options_path.is_file() {
            return Some(yaml_options_path);
        }
    }
    return None;
}

pub trait GroupConfigurationLoader {
    fn load<P: AsRef<Path>>(&self, path: P) -> Result<GroupConfiguration>;
}

pub struct DefaultGroupConfigurationLoader {}

impl DefaultGroupConfigurationLoader {
    pub fn new() -> Self {
        DefaultGroupConfigurationLoader {}
    }
}

impl GroupConfigurationLoader for DefaultGroupConfigurationLoader {
    fn load<P: AsRef<Path>>(&self, workspace: P) -> Result<GroupConfiguration> {
        return match detect_configuration_path(workspace) {
            Some(path) => match path.extension() {
                Some(extemsion) => match extemsion.to_str() {
                    Some("json") => {
                        let loader = JsonGroupConfigurationLoader {};
                        let file_configuration = loader.load(path)?;
                        Ok(file_configuration)
                    }
                    Some("toml") => {
                        let loader = TomlGroupConfigurationLoader {};
                        let file_configuration = loader.load(path)?;
                        Ok(file_configuration)
                    }
                    Some("yaml") => {
                        let loader = YamlGroupConfigurationLoader {};
                        let file_configuration = loader.load(path)?;
                        Ok(file_configuration)
                    }
                    _ => Err(Error::new("Not found configuration file")),
                },
                None => Err(Error::new("Not found configuration file")),
            },
            None => Err(Error::new("Not found configuration file")),
        };
    }
}

pub struct JsonGroupConfigurationLoader {}

impl GroupConfigurationLoader for JsonGroupConfigurationLoader {
    fn load<P: AsRef<Path>>(&self, path: P) -> Result<GroupConfiguration> {
        let path = path.as_ref().absolutize().unwrap();
        let content = std::fs::read_to_string(path)
            .map_err(|e| Error::new("Failed to load configuration file.").with_inner_error(&e))?;
        return serde_json::from_str::<GroupConfiguration>(&content)
            .map_err(|e| Error::new("Failed to convert configuration.").with_inner_error(&e));
    }
}

pub struct TomlGroupConfigurationLoader {}

impl GroupConfigurationLoader for TomlGroupConfigurationLoader {
    fn load<P: AsRef<Path>>(&self, path: P) -> Result<GroupConfiguration> {
        let path = path.as_ref().absolutize().unwrap();
        let content = std::fs::read_to_string(path)
            .map_err(|e| Error::new("Failed to load configuration file.").with_inner_error(&e))?;
        return toml::from_str::<GroupConfiguration>(&content)
            .map_err(|e| Error::new("Failed to convert configuration.").with_inner_error(&e));
    }
}

pub struct YamlGroupConfigurationLoader {}

impl GroupConfigurationLoader for YamlGroupConfigurationLoader {
    fn load<P: AsRef<Path>>(&self, path: P) -> Result<GroupConfiguration> {
        let path = path.as_ref().absolutize().unwrap();
        let content = std::fs::read_to_string(path)
            .map_err(|e| Error::new("Failed to load configuration file.").with_inner_error(&e))?;
        return serde_yaml::from_str::<GroupConfiguration>(&content)
            .map_err(|e| Error::new("Failed to convert configuration.").with_inner_error(&e));
    }
}
