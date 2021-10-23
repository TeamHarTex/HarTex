//! # The `discord` Module
//!
//! This module contains re-exports of *most* of the `twilight` ecosystem of crates to reduce the
//! need to add the `twilight_*` dependencies to the `Cargo.toml`s of the individual separated
//! HarTex crates.

/// Re-export `twilight_embed_builder`
///
/// A set of builders for the `twilight-rs` ecosystem for creating message embeds and are useful
/// when creating or updating messages.
pub use twilight_embed_builder as embed_builder;

/// Re-export `twilight_model`
///
/// A crate of `serde` models defining the Discord APIs with a few convenience methods implemented
/// on top of them for the `twilight-rs` ecosystem.
pub use twilight_model as model;

/// Re-export `twilight_standby`
///
/// Standby is a utility crate for the `twilight-rs` ecosystem to wait for an event to happen based
/// on a predicate check. For example, you may have a command that has a reaction menu of ✅ and ❌.
/// If you want to handle a reaction to these, using something like an application-level state or
/// event stream may not suit your use case. It may be cleaner to wait for a reaction inline to
/// your function.
pub use twilight_standby as standby;

/// # Module `cache_inmemory`
///
/// Re-export of `twilight_cache_inmemory` and a clone-able wrapper of `InMemoryCache`.
pub mod cache_inmemory {
    use std::{
        ops::Deref,
        sync::Arc
    };

    pub use twilight_cache_inmemory::*;

    #[derive(Clone)]
    pub struct CloneableInMemoryCache(Arc<InMemoryCache>);

    impl CloneableInMemoryCache {
        pub fn new(cache: InMemoryCache) -> Self {
            Self(Arc::new(cache))
        }
    }

    impl Deref for CloneableInMemoryCache {
        type Target = InMemoryCache;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}

/// # Module `mention`
///
/// Re-export of `twilight_gateway` and a clone-able wrapper of `Cluster`.
pub mod gateway {
    use std::{
        ops::Deref,
        sync::Arc
    };

    pub use twilight_gateway::*;

    #[derive(Clone)]
    pub struct CloneableCluster(Arc<Cluster>);

    impl CloneableCluster {
        pub fn new(cluster: Cluster) -> Self {
            Self(Arc::new(cluster))
        }
    }

    impl Deref for CloneableCluster {
        type Target = Cluster;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}

/// # Module `http`
///
/// Re-export of `twilight_http` and a clone-able wrapper of `Client`.
pub mod http {
    use std::{
        ops::Deref,
        sync::Arc
    };

    pub use twilight_http::*;

    #[derive(Clone)]
    pub struct CloneableClient(Arc<Client>);

    impl CloneableClient {
        pub fn new(client: Client) -> Self {
            Self(Arc::new(client))
        }
    }

    impl Deref for CloneableClient {
        type Target = Client;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}

/// # Module `util`
///
/// Re-export of `twilight_util` and `twilight_mention`.
pub mod util {
    pub use twilight_mention as mention;
    pub use twilight_util::*;
}
