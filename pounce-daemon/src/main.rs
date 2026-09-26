use anyhow::{Ok, Result};
use nanologger::{LogLevel, LoggerBuilder, debug, info, trace};
use pounce_config::{node::NodeConfig, server::ServerConfig, traits::LoadableConfig};

const DEFAULT_NODE_CONFIG_PATH: &str = "config/node.toml";

fn main() -> Result<()> {
    LoggerBuilder::new()
        .level(if cfg!(debug_assertions) {
            LogLevel::Trace
        } else {
            LogLevel::Info
        })
        .timestamps(true)
        .thread_info(true)
        .init()
        .unwrap();

    let node_config_path =
        std::env::var("POUNCE_NODE_CONFIG").unwrap_or_else(|_| DEFAULT_NODE_CONFIG_PATH.into());

    trace!("Loading node config {}", &node_config_path);
    let node = NodeConfig::load_from_toml(&node_config_path)?;
    info!(
        "Loaded node '{}' ({:.8}...) - API {}:{}",
        node.name,
        node.uuid.to_string(),
        node.api.host,
        node.api.port
    );

    trace!("Loading servers from {:?}", &node.system.servers_directory);
    let servers = ServerConfig::load_all_from_dir(&node.system.servers_directory)?;
    info!("Loaded {} server", servers.len());

    for server in &servers {
        debug!(
            "- {} ({:.8}...) [autostart={}, image={}, port={}]",
            server.name,
            server.uuid.to_string(),
            server.autostart,
            server.container.image,
            server.allocations.primary_port
        )
    }

    Ok(())
}
