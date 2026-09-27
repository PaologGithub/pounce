use std::{collections::HashMap, sync::Arc};

use anyhow::{Result, anyhow};
use tokio::sync::{broadcast, mpsc};
use tokio::task::JoinHandle;
use uuid::Uuid;

use pounce_config::server::ServerConfig;

use crate::{
    backend::Backend,
    event::ServerEvent,
    server::{ManagedServer, ServerCommand},
};

pub struct ServerManager {
    commands: HashMap<Uuid, mpsc::Sender<ServerCommand>>,
    events: HashMap<Uuid, broadcast::Sender<ServerEvent>>,
    handles: HashMap<Uuid, JoinHandle<()>>,
}

impl ServerManager {
    pub fn load_from(configs: Vec<ServerConfig>, backend: Arc<dyn Backend>) -> Self {
        let mut manager = Self {
            commands: HashMap::new(),
            events: HashMap::new(),
            handles: HashMap::new(),
        };

        for config in configs {
            let uuid = config.uuid;
            let (command_tx, command_rx) = mpsc::channel(32);
            let (event_tx, _) = broadcast::channel(64);

            let mut server =
                ManagedServer::new(config, backend.clone(), command_rx, event_tx.clone());
            let handle = tokio::spawn(async move {
                server.update_loop().await;
            });

            manager.commands.insert(uuid, command_tx);
            manager.events.insert(uuid, event_tx);
            manager.handles.insert(uuid, handle);
        }

        manager
    }

    pub async fn start(&self, id: Uuid) -> Result<()> {
        self.commands
            .get(&id)
            .ok_or_else(|| anyhow!("Unknown server: {id}"))?
            .send(ServerCommand::Start)
            .await?;
        Ok(())
    }

    pub async fn stop(&self, id: Uuid) -> Result<()> {
        self.commands
            .get(&id)
            .ok_or_else(|| anyhow!("Unknown server: {id}"))?
            .send(ServerCommand::Stop)
            .await?;
        Ok(())
    }

    pub fn subscribe(&self, id: Uuid) -> Option<broadcast::Receiver<ServerEvent>> {
        self.events.get(&id).map(|tx| tx.subscribe())
    }
}
