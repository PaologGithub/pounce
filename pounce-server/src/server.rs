use std::sync::mpsc::Receiver;

use pounce_config::server::ServerConfig;

use crate::{event::ServerEvent, state::ServerState};

/// Managed server (with config, state, events)
pub struct ManagedServer {
    pub config:   ServerConfig,
    pub state:    ServerState,
    pub receiver: Receiver<ServerEvent>
}

impl ManagedServer {
    pub fn new(config: ServerConfig, receiver: Receiver<ServerEvent>) -> Self {
        Self {
            config,
            state: ServerState::Stopped,
            receiver
        }
    }

    pub async fn update_loop(&mut self) {
        while let Ok(_event) = self.receiver.recv() {

        }
    }
}