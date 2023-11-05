/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use hartex_bors_database::client::SeaORMDatabaseClient;
use leptos::component;
use leptos::logging;
use leptos::view;
use leptos::IntoView;
use leptos_router::Route;
use leptos_router::Router;
use leptos_router::Routes;

use crate::home::Home;
use crate::DatabaseClientContext;
use crate::DATABASE;

#[component]
pub fn App() -> impl IntoView {
    logging::log!("rendering App component");

    leptos::spawn_local(|_| (async {
        let client = SeaORMDatabaseClient::new(
            hartex_bors_database::initialize_database(false)
                .await
                .unwrap(),
        );
        DATABASE.set(client).ok();
    })());

    leptos::provide_context(DatabaseClientContext(DATABASE.get().unwrap().clone()));

    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                </Routes>
            </main>
        </Router>
    }
}
