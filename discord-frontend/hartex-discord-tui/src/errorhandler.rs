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

use std::{panic, process};

use color_eyre::{config::HookBuilder, eyre::Result};
use tracing::error;

use crate::tui::Tui;

pub fn initialize() -> Result<()> {
    let (panic_hook, eyre_hook) = HookBuilder::new()
        .capture_span_trace_by_default(false)
        .display_env_section(false)
        .display_location_section(false)
        .into_hooks();

    eyre_hook.install()?;

    panic::set_hook(Box::new(move |info| {
        if let Ok(mut tui) = Tui::new() {
            if let Err(e) = tui.exit() {
                error!("unable to exit terminal: {e:?}")
            }
        }

        let msg = format!("{}", panic_hook.panic_report(info));
        error!("{msg}");

        process::exit(1)
    }));

    Ok(())
}
