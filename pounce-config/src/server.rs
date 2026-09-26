use std::path::PathBuf;
use std::{collections::HashMap, path::Path};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::ConfigError;
use crate::traits::LoadableConfig;

/// Configuration for single-server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Server's UUID
    pub uuid: Uuid,

    /// Server's name
    pub name: String,

    /// Server's description (optional)
    #[serde(default)]
    pub description: String,

    /// If the server is started with the daemon
    #[serde(default)]
    pub autostart: bool,

    /// Container config
    pub container: ContainerConfig,

    /// Resource limits
    pub limits: ResourceLimits,

    /// Network allocations
    pub allocations: NetworkAllocations,
}

/// Container config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerConfig {
    /// Docker image
    pub image: String,

    /// Startup command
    pub startup_command: String,

    /// Process' working directory (from the server data dir)
    ///  => `.` -> `{SERVER_DATA}/.` -> `/var/lib/pounce/data/{SERVER}/.`
    pub working_directory: PathBuf,

    /// Environment variable
    pub environment: HashMap<String, String>,
}

/// Resource limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Memory limit (megabytes, `0` -> unlimited)
    #[serde(default)]
    pub memory_mb: u64,

    /// CPU limit (`100` -> 1 core, `0` -> unlimited)
    #[serde(default)]
    pub cpu_percent: u32,

    /// Disk limit (megabytes, `0` -> unlimited)
    #[serde(default)]
    pub disk_mb: u64,
}

/// Network allocations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkAllocations {
    /// Primary port
    pub primary_port: u16,

    /// Additional exposed port
    #[serde(default)]
    pub additional_ports: Vec<u16>,
}

impl LoadableConfig for ServerConfig {}
impl ServerConfig {
    /// Load every server from a path
    pub fn load_all_from_dir(dir: impl AsRef<Path>) -> Result<Vec<Self>, ConfigError> {
        let dir = dir.as_ref();
        let entries = std::fs::read_dir(dir).map_err(|err| ConfigError::IOError {
            path: dir.to_path_buf(),
            err,
        })?;

        let mut servers = Vec::new();

        for entry in entries {
            let entry = entry.map_err(|err| ConfigError::IOError {
                path: dir.to_path_buf(),
                err,
            })?;
            let path = entry.path();

            if path.extension().and_then(|ext| ext.to_str()) == Some("toml") {
                servers.push(Self::load_from_toml(&path)?);
            }
        }

        Ok(servers)
    }
}
