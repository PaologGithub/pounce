use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::traits::LoadableConfig;

/// Configuration for the node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Node's unique ID
    pub uuid: Uuid,

    /// Node's name
    pub name: String,

    /// Node's description (optional)
    #[serde(default)]
    pub description: String,

    /// Network API's config
    pub api: APIConfig,

    /// FS-related's config
    pub system: SystemConfig,
}

/// Configuration for the networking API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIConfig {
    /// API's ip address
    #[serde(default = "default_host")]
    pub host: String,

    /// API's port
    #[serde(default = "default_port")]
    pub port: u16,

    /// Is SSL enabled?
    #[serde(default)]
    pub ssl_enabled: bool,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    8443
}

/// Configuration for the interactions between node & system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    /// Root directory
    pub data_directory: PathBuf,

    /// Server config directory
    pub servers_directory: PathBuf,
}

impl LoadableConfig for NodeConfig {}
