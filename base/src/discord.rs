/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

//! Re-exports of the `twilight-*` ecosystem libraries.

pub use twilight_embed_builder as embed_builder;
pub use twilight_model as model;
pub use twilight_standby as standby;

pub mod cache_inmemory {
    use std::{
        ops::Deref,
        sync::Arc
    };

    pub use twilight_cache_inmemory::*;

    /// A cloneable in-memory cache.
    ///
    /// This is a wrapper of the `twilight` [`InMemoryCache`] - where a breaking change removed the
    /// clone-ability of the structure as the inner data is no longer wrapped in an [`Arc`] - which
    /// wrappers the structure itself in an [`Arc`], hence making it cloneable.
    ///
    /// [`Arc`]: https://doc.rust-lang.org/nightly/std/sync/struct.Arc.html
    /// [`InMemoryCache`]: https://api.twilight.rs/twilight_cache_inmemory/struct.InMemoryCache.html
    #[derive(Clone)]
    pub struct CloneableInMemoryCache(pub Arc<InMemoryCache>);

    impl CloneableInMemoryCache {
        /// Constructs a new cloneable in-memory cache from a not-cloneable [`InMemoryCache`].
        ///
        /// [`InMemoryCache`]: https://api.twilight.rs/twilight_cache_inmemory/struct.InMemoryCache.html
        #[must_use]
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

    /// A cloneable gateway cluster.
    ///
    /// This is a wrapper of the `twilight` [`Cluster`] - where a breaking change removed the
    /// clone-ability of the structure as the inner data is no longer wrapped in an [`Arc`] - which
    /// wrappers the structure itself in an [`Arc`], hence making it cloneable.
    ///
    /// [`Arc`]: https://doc.rust-lang.org/nightly/std/sync/struct.Arc.html
    /// [`Cluster`]: https://api.twilight.rs/twilight_gateway/cluster/struct.Cluster.html
    #[derive(Clone)]
    pub struct CloneableCluster(pub Arc<Cluster>);

    impl CloneableCluster {
        /// Constructs a new cloneable gateway cluster from a not-cloneable [`Cluster`].
        ///
        /// [`Cluster`]: https://api.twilight.rs/twilight_gateway/cluster/struct.Cluster.html
        #[must_use]
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

pub mod http {
    use std::{
        ops::Deref,
        sync::Arc
    };

    pub use twilight_http::*;

    /// A cloneable HTTP REST client.
    ///
    /// This is a wrapper of the `twilight` [`Client`] - where a breaking change removed the
    /// clone-ability of the structure as the inner data is no longer wrapped in an [`Arc`] - which
    /// wrappers the structure itself in an [`Arc`], hence making it cloneable.
    ///
    /// [`Arc`]: https://doc.rust-lang.org/nightly/std/sync/struct.Arc.html
    /// [`Client`]: https://api.twilight.rs/twilight_http/client/struct.Client.html
    #[derive(Clone)]
    pub struct CloneableClient(pub Arc<Client>);

    impl CloneableClient {
        /// Constructs a new cloneable gateway cluster from a not-cloneable [`Client`].
        ///
        /// [`Client`]: https://api.twilight.rs/twilight_http/client/struct.Client.html
        #[must_use]
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

/// Re-export of `twilight-util` and `twilight-mention`.
pub mod util {
    pub use twilight_mention as mention;
    pub use twilight_util::*;
}
