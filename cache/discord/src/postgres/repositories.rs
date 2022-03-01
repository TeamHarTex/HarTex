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
    fn upsert(&self, current_user: CachedCurrentUser) -> UpsertEntityFuture<'_, PostgresBackendError> {
        Box::pin(async {
            let pgsql_creds = env::var("PGSQL_CACHE_DB_CREDENTIALS")?;
            let mut pool = PgPool::connect(&pgsql_creds).await?;
            let query = include_str!("../../include/postgres/repositories/current_user/upsert.sql");

            sqlx::query(query)
                .bind(current_user.id.to_string())
                .bind(current_user.username)
                .bind(current_user.discriminator)
                .bind(current_user.avatar)
                .bind(current_user.bot)
                .bind(current_user.system)
                .bind(current_user.mfa_enabled)
                .bind(current_user.banner)
                .bind(current_user.accent_colour)
                .bind(current_user.locale)
                .bind(current_user.verified)
                .bind(current_user.email)
                .bind(current_user.flags)
                .bind(current_user.premium_type)
                .bind(current_user.public_flags)
                .execute(&mut pool)
                .await?;

            Ok(())
        })
    }
}
