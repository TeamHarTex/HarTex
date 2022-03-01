use base::discord::model::gateway::payload::incoming::Ready;
use cache_base::Repository;

use crate::entities::users::CachedCurrentUser;
use crate::postgres::PostgresBackend;
use crate::repositories::CurrentUserRepository;

impl CacheUpdatable<PostgresBackend> for Ready {
    fn update(&self, _: &DiscordCache) -> UpdateCacheFuture<'_, crate::postgres::PostgresBackend> {
        let current_user = CachedCurrentUser::from(self.user.clone());

        CurrentUserRepository.upsert(current_user)
    }
}
