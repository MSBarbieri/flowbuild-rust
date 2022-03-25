#[macro_use]
extern crate lazy_static;
extern crate pretty_env_logger;
extern crate log;

#[macro_use]
extern crate diesel;

pub use anyhow::Result;
pub mod core;
pub use crate::core::*;

pub mod schema;
pub mod models;

#[cfg(feature = "cockpit")]
pub mod cockpit;
#[cfg(feature = "cockpit")]
pub use crate::cockpit::*;
