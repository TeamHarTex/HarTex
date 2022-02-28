//! The PostgreSQL cache backend.

use cache_base::Backend;

/// The PostgreSQL cache backend implementation.
pub struct PostgresBackend;

impl Backend for PostgresBackend {
    type Error = ();
}
