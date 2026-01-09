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

use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use lazy_static::lazy_static;

use crate::app::{Action, Menu};

lazy_static! {
    pub static ref KEYBINDS: HashMap<Menu, HashMap<Vec<KeyEvent>, Action>> = {
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

        map
    };
}
