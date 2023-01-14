use hartex_discord_entitycache_core::Entity;
pub struct DeriveMultipleId {
    #[entity(id)]
    pub id1: u64,
    #[entity(id)]
    pub id2: u64,
}
extern crate hartex_discord_entitycache_core as _entitycache_core;
#[automatically_derived]
impl _entitycache_core::traits::Entity for DeriveMultipleId {
    type Id = (u64, u64);
    fn id(&self) -> <Self as _entitycache_core::traits::Entity>::Id {
        (self.id1, self.id2)
    }
}
