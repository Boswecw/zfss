//! ZFSS Database module

pub mod pool;

pub use pool::{create_pool, health_check, run_migrations};
