use hartex_discord_entitycache_core::Entity;
pub struct DeriveSingleId {
    #[entity(id)]
    pub id: u64,
}
const _: () = {
    extern crate hartex_discord_entitycache_core as _entitycache_core;
    #[automatically_derived]
    impl _entitycache_core::traits::Entity for DeriveSingleId {
        type Id = u64;
        fn id(&self) -> <Self as _entitycache_core::traits::Entity>::Id {
            self.id
        }
    }
};
