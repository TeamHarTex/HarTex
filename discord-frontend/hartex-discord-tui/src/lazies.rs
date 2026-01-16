/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2026 HarTex Project Developers
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

use std::{collections::HashMap, sync::LazyLock};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::{
    app::{Action, Menu},
    component::{
        Component,
        pages::{overview::OverviewPage, shards::ShardsPage},
        tab_selector::Tab,
    },
};

pub static KEYBINDS: LazyLock<HashMap<Menu, HashMap<Vec<KeyEvent>, Action>>> =
    LazyLock::new(|| {
        let mut map = HashMap::new();

        let entry: &mut HashMap<_, _> = map.entry(Menu::Main).or_default();
        entry.insert(
            vec![KeyEvent::new_with_kind(
                KeyCode::Char('q'),
                KeyModifiers::CONTROL,
                KeyEventKind::Press,
            )],
            Action::Quit,
        );
        entry.insert(
            vec![KeyEvent::new_with_kind(
                KeyCode::Down,
                KeyModifiers::NONE,
                KeyEventKind::Press,
            )],
            Action::Next,
        );
        entry.insert(
            vec![KeyEvent::new_with_kind(
                KeyCode::Up,
                KeyModifiers::NONE,
                KeyEventKind::Press,
            )],
            Action::Previous,
        );

        map
    });

pub static PAGES: LazyLock<HashMap<Tab, Box<dyn Component + Send + Sync>>> = LazyLock::new(|| {
    let mut map: HashMap<Tab, Box<dyn Component + Send + Sync>> = HashMap::new();

    map.insert(Tab::Overview, Box::new(OverviewPage::new()));
    map.insert(Tab::Shards, Box::new(ShardsPage));

    map
});
