use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Couldn't read config file at {path}: {err}")]
    IOError {
        path: PathBuf,
        #[source]
        err: std::io::Error,
    },

    #[error("Couldn't parse TOML in {path}: {err}")]
    ParseError {
        path: PathBuf,
        #[source]
        err: toml::de::Error,
    },

    #[error("Couldn't serialize TOML: {0}")]
    Serialize(#[from] toml::ser::Error),
}
