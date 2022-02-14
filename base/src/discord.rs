/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
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

#[cfg(feature = "discord-gateway")]
pub mod gateway {
    pub(self) use std::ops::Deref;
    pub(self) use std::sync::Arc;

    pub use twilight_gateway::*;

    #[allow(clippy::module_name_repetitions)]
    #[derive(Clone)]
    pub struct GatewayCluster(Arc<Cluster>);

    impl GatewayCluster {
        #[must_use]
        pub fn new(cluster: Cluster) -> Self {
            Self(Arc::new(cluster))
        }
    }

    impl Deref for GatewayCluster {
        type Target = Cluster;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}

#[cfg(feature = "discord-model")]
pub use twilight_model as model;
