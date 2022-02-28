//! Repositories in the PostgreSQL backend.

use cache_base::future::UpsertEntityFuture;
use cache_base::{Backend, Repository};

use crate::entities::users::CachedCurrentUser;
use crate::postgres::error::PostgresBackendError;
use crate::postgres::PostgresBackend;

pub struct CurrentUserRepository;

impl Repository<PostgresBackend, CachedCurrentUser> for CurrentUserRepository {
    fn upsert(
        &self,
        current_user: CachedCurrentUser,
    ) -> UpsertEntityFuture<'_, PostgresBackendError> {
        Box::pin(async {
            Ok(())
        })
    }
}
