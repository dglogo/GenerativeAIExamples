//! Main Crate Error

pub type Result<T> = core::result::Result<T, Error>;

// // Error handling with thiserror::Error

// #[derive(Debug, thiserror::Error)]
// pub enum Error {
//     /// For starter, to remove as code matures.
//     #[error("Generic error: {0}")]
//     Generic(String),

//     /// For starter, to remove as code matures.
//     #[error("Static error: {0}")]
//     Static(&'static str),

//     #[error(transparent)]
//     IO(#[from] std::io::Error),
// }

use derive_more::From;

#[derive(Debug, From)]
pub enum Error {
    /// For starter, to remove as code matures.
    #[from]
    Custom(String),

    // -- Externals
    #[from]
    Json(serde_json::Error),

    #[from]
    Io(std::io::Error),
}

impl From<&str> for Error {
    fn from(val: &str) -> Self {
        Self::Custom(val.to_string())
    }
}
