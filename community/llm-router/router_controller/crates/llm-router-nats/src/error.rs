//! Main Crate Error

#[derive(Debug, thiserror::Error)]
pub enum NatsError {
    /// For starter, to remove as code matures.
    #[error("Generic error: {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}
