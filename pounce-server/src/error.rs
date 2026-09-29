#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error("Couldn't parse startup command for {name}: {err}")]
    StartupCommandParseError {
        name: String,
        err: String
    },

    #[error("Couldn't start server {name}: {err}")]
    StartupError {
        name: String,
        #[source]
        err: tokio::io::Error
    },

    #[error("Couldn't stop server {name}: {err}")]
    StopError {
        name: String,
        #[source]
        err: tokio::io::Error
    },

    #[error("Couldn't kill server {name}: {err}")]
    KillError {
        name: String,
        #[source]
        err: tokio::io::Error
    },

    #[error("Couldn't get status for server {name}: {err}")]
    StatusError {
        name: String,
        #[source]
        err: tokio::io::Error
    },
}
