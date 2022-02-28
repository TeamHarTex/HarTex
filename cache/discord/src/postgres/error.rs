//! Errors for the PostgreSQL cache backend.

use std::env::VarError;

use sqlx::Error as SqlxError;

/// Error generated from a backend operation.
#[allow(clippy::module_name_repetitions)]
pub enum PostgresBackendError {
    /// An error occurred whilst performing environment variable operations.
    EnvVar { src: VarError },
    /// An error occurred whilst performing database operations.
    Sqlx { src: SqlxError },
}

impl From<SqlxError> for PostgresBackendError {
    fn from(src: SqlxError) -> Self {
        Self::Sqlx { src }
    }
}

impl From<VarError> for PostgresBackendError {
    fn from(src: VarError) -> Self {
        Self::EnvVar { src }
    }
}
