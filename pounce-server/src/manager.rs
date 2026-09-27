use std::{collections::HashMap, sync::{Arc, mpsc::{self, Receiver, Sender}}};

use pounce_config::server::ServerConfig;
use uuid::Uuid;

use crate::{backend::backend::Backend, event::ServerEvent, server::ManagedServer};

pub struct ServerManager {
    _backend: Arc<dyn Backend>,
    servers: HashMap<Uuid, ManagedServer>,
    senders: HashMap<Uuid, Sender<ServerEvent>>
}

impl ServerManager {
    pub fn load_from(configs: Vec<ServerConfig>, backend: Arc<dyn Backend>) -> Self {
        let mut manager = Self {
            _backend: backend,
            servers: HashMap::new(),
            senders: HashMap::new()
        };

        for config in configs {
            let (rx, tx): (Sender<ServerEvent>, Receiver<ServerEvent>) = mpsc::channel();
            
            let uuid = config.uuid;
            let server = ManagedServer::new(config, tx);

            manager.servers.insert(uuid, server);
            manager.senders.insert(uuid, rx);
        }

        manager
    }
}