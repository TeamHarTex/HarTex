use cache_base::Repository;

use crate::entities::users::CachedCurrentUser;
use crate::repositories::CurrentUserRepository;

impl CacheUpdatable<crate::postgres::PostgresBackend> for base::discord::model::gateway::payload::incoming::Ready {
    fn update(&self, _: &DiscordCache) -> UpdateCacheFuture<'_, crate::postgres::PostgresBackend> {
        let current_user = CachedCurrentUser::from(self.user.clone());

        CurrentUserRepository.upsert(current_user)
    }
}
