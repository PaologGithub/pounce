use std::sync::Arc;

use pounce_config::server::ServerConfig;
use tokio::sync::{broadcast::Sender, mpsc::Receiver, oneshot};

use crate::{backend::Backend, event::ServerEvent, state::ServerState};

pub enum ServerCommand {
    Start,
    Stop,
    Kill,
    Shutdown,
    GetState(oneshot::Sender<ServerState>),
}

/// Managed server (with config, state, events)
pub struct ManagedServer {
    pub config: ServerConfig,
    pub state: ServerState,
    backend: Arc<dyn Backend>,
    commands_rx: Receiver<ServerCommand>,
    events_tx: Sender<ServerEvent>,
}

impl ManagedServer {
    pub fn new(
        config: ServerConfig,
        backend: Arc<dyn Backend>,
        commands_rx: Receiver<ServerCommand>,
        events_tx: Sender<ServerEvent>,
    ) -> Self {
        Self {
            config,
            state: ServerState::Stopped,
            backend,
            commands_rx,
            events_tx,
        }
    }

    pub async fn update_loop(&mut self) {
        while let Some(cmd) = self.commands_rx.recv().await {
            match cmd {
                ServerCommand::Start => self.handle_start().await,
                ServerCommand::Stop => self.handle_stop().await,
                ServerCommand::Kill => self.handle_kill().await,
                ServerCommand::GetState(reply) => {
                    let _ = reply.send(self.state);
                }
                ServerCommand::Shutdown => break,
            }
        }
    }

    async fn handle_start(&mut self) {
        if matches!(self.state, ServerState::Running | ServerState::Starting) {
            return;
        }
        self.set_state(ServerState::Starting);

        let backend = self.backend.clone();
        match backend.start(self).await {
            Ok(()) => self.set_state(ServerState::Running),
            Err(e) => {
                let _ = self.events_tx.send(ServerEvent::Crashed {
                    reason: e.to_string(),
                });
                self.set_state(ServerState::Crashed);
            }
        }
    }

    async fn handle_stop(&mut self) {
        self.set_state(ServerState::Stopping);
        let backend = self.backend.clone();
        if let Err(e) = backend.stop(self).await {
            let _ = self.events_tx.send(ServerEvent::Crashed {
                reason: e.to_string(),
            });
        }
        self.set_state(ServerState::Stopped);
    }

    async fn handle_kill(&mut self) {
        let backend = self.backend.clone();
        let _ = backend.kill(self).await;
        self.set_state(ServerState::Stopped);
    }

    fn set_state(&mut self, new: ServerState) {
        self.state = new;
        let _ = self.events_tx.send(ServerEvent::StateChanged(new));
    }
}
