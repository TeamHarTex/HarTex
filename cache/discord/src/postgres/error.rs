//! Errors for the PostgreSQL cache backend.

use sqlx::Error as SqlxError;

/// Error generated from a backend operation.
#[allow(clippy::module_name_repetitions)]
pub enum PostgresBackendError {
    /// An error occurred whilst performing database operations.
    Sqlx { src: SqlxError },
}

impl From<SqlxError> for PostgresBackendError {
    fn from(src: SqlxError) -> Self {
        Self::Sqlx { src }
    }
}
