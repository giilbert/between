#![allow(unused_imports)]
pub type Result<T, E = eyre::Report> = eyre::Result<T, E>;
pub use eyre::eyre;
pub use tracing::{debug, error, info, trace, warn};
