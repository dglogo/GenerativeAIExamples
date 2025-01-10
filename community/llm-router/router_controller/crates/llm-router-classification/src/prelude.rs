//! Crate prelude

pub use crate::error::{Error, Result};

// Generic Wrapper tuple struct for newtype pattern
pub struct W<T>(pub T);
