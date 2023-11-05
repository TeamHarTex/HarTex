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

use hartex_bors_core::DatabaseClient;
use leptos::component;
use leptos::logging;
use leptos::view;
use leptos::CollectView;
use leptos::ErrorBoundary;
use leptos::Errors;
use leptos::IntoView;
use leptos::Resource;
use leptos::RwSignal;
use leptos::Transition;

use crate::DatabaseClientContext;

#[component]
pub fn Home() -> impl IntoView {
    logging::log!("rendering Home component");

    let context = leptos::expect_context::<DatabaseClientContext>();
    let resource = Resource::once(|_| async { context.0.get_repositories().await });

    let error_fallback = move |_: RwSignal<Errors>| {
        view! {
            <p>"Could not load repositories due to an error."</p>
        }
    };

    let loading_fallback = move || {
        view! {
            <p>"Loading repositories..."</p>
        }
    };

    let repositories_view = move || {
        resource.and_then(|data| {
            data.iter()
                .map(|repository| view! {
                <li><a href={format!("queue/{}", repository.name.clone())}>{repository.name.clone()}</a></li>
            })
                .collect_view()
        })
    };

    view! {
        <h1>"Bors V2"</h1>

        <h2>"Repositories"</h2>
        <ErrorBoundary fallback=error_fallback>
            <Transition fallback=loading_fallback>
                <ul>{repositories_view}</ul>
            </Transition>
        </ErrorBoundary>
    }
}
