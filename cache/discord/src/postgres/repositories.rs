//! Repositories in the PostgreSQL backend.

use std::env;

use cache_base::future::UpsertEntityFuture;
use cache_base::{Backend, Repository};
use sqlx::postgres::PgPool;

use crate::entities::users::CachedCurrentUser;
use crate::postgres::error::PostgresBackendError;
use crate::postgres::PostgresBackend;

pub struct CurrentUserRepository;

impl Repository<PostgresBackend, CachedCurrentUser> for CurrentUserRepository {
    fn upsert(
        &self,
        _: CachedCurrentUser,
    ) -> UpsertEntityFuture<'_, PostgresBackendError> {
        Box::pin(async {
            let pgsql_creds = env::var("PGSQL_CACHE_DB_CREDENTIALS")?;
            let _ = PgPool::connect(&pgsql_creds).await?;
            let _ = include_str!("../../../postgres/repositories/current_user/upsert.sql");

            Ok(())
        })
    }
}
