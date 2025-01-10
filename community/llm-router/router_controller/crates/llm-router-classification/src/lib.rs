#![allow(unused)] // For starter.

//! Lib mode to allow the examples/ to import those modules.

pub mod error;

pub use error::{Error, Result};

pub mod prelude;
pub mod utils;
