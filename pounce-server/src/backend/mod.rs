pub mod vanilla;

use crate::{error::BackendError, server::ManagedServer};

#[async_trait::async_trait]
pub trait Backend: Send + Sync {
    fn add_server(&self, server: &ManagedServer) -> Result<(), BackendError>;

    async fn start(&self, server: &ManagedServer) -> Result<(), BackendError>;
    async fn stop(&self, server: &ManagedServer) -> Result<(), BackendError>;
    async fn kill(&self, server: &ManagedServer) -> Result<(), BackendError>;
    async fn is_running(&self, server: &ManagedServer) -> Result<bool, BackendError>;
}
