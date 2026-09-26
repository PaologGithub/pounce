use std::path::Path;

use crate::error::ConfigError;
use serde::de::DeserializeOwned;

pub trait LoadableConfig: Sized + DeserializeOwned {
    fn load_from_toml(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let raw = std::fs::read_to_string(path).map_err(|err| ConfigError::IOError {
            path: path.to_path_buf(),
            err,
        })?;

        toml::from_str(&raw).map_err(|err| ConfigError::ParseError {
            path: path.to_path_buf(),
            err,
        })
    }
}
