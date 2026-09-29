use std::collections::HashMap;

use send_ctrlc::{Interruptible, InterruptibleCommand, tokio::InterruptibleChild};
use tokio::{process::Command, sync::Mutex};
use uuid::Uuid;

use crate::{backend::Backend, error::BackendError, server::ManagedServer};

/// Server backend that simply uses the OS's multiprocess
pub struct VanillaBackend {
    servers: Mutex<HashMap<Uuid, InterruptibleChild>>
}

impl Default for VanillaBackend {
    fn default() -> Self {
        Self {
            servers: Mutex::new(HashMap::new())
        }
    }
}

#[async_trait::async_trait]
impl Backend for VanillaBackend {
    fn add_server(&self, _server: &ManagedServer) -> Result<(), BackendError> {
        Ok(())
    }


    async fn start(&self, server: &ManagedServer) -> Result<(), BackendError> {
        let split: Vec<&str> = server
            .config
            .container
            .startup_command
            .split_whitespace()
            .collect();

        let (process, args) = split.split_first().ok_or_else(|| {
            BackendError::StartupCommandParseError {
                name: server.config.name.clone(),
                err: "startup command cannot be empty".to_string(),
            }
        })?;

        let mut command = Command::new(*process);
        command
            .args(args)
            .envs(&server.config.container.environment)
            .current_dir(&server.config.container.working_directory);

        let child = command
            .spawn_interruptible()
            .map_err(|err| BackendError::StartupError {
                name: server.config.name.clone(),
                err,
            })?;
        
        self.servers.lock().await.insert(server.config.uuid, child);

        Ok(())
    }

    async fn stop(&self, server: &ManagedServer) -> Result<(), BackendError> {
        let mut servers = self.servers.lock().await;
        if let Some(child) = servers.get_mut(&server.config.uuid) {
            child
                .interrupt()
                .map_err(|err| BackendError::StopError {
                    name: server.config.name.clone(),
                    err,
                })?;
        }
        Ok(())
    }

    async fn kill(&self, server: &ManagedServer) -> Result<(), BackendError> {
        let mut servers = self.servers.lock().await;
        if let Some(child) = servers.get_mut(&server.config.uuid) {
            child
                .kill()
                .await
                .map_err(|err| BackendError::KillError {
                    name: server.config.name.clone(),
                    err,
                })?;
        }
        Ok(())
    }

    async fn is_running(&self, server: &ManagedServer) -> Result<bool, BackendError> {
        let mut servers = self.servers.lock().await;
        let Some(child) = servers.get_mut(&server.config.uuid) else {
            return Ok(false);
        };

        match child.try_wait() {
            Ok(Some(_)) => Ok(false),
            Ok(None) => Ok(true),
            Err(err) => Err(BackendError::StatusError {
                name: server.config.name.clone(),
                err,
            }),
        }
    }
}