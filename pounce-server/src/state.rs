#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Crashed,
}
