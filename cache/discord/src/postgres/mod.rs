//! The PostgreSQL cache backend.

use cache_base::Backend;

pub mod error;

/// The PostgreSQL cache backend implementation.
pub struct PostgresBackend;

impl Backend for PostgresBackend {
    type Error = error::PostgresBackendError;
}
