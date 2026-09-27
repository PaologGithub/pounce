use crate::state::ServerState;

#[derive(Debug, Clone)]
pub enum ServerEvent {
    StateChanged(ServerState),
    Crashed { reason: String },
}
