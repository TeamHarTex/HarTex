/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2024 HarTex Project Developers
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

#[allow(clippy::module_name_repetitions)]
pub trait MarkdownStyle {
    #[must_use]
    fn discord_bold(self) -> Self;

    #[must_use]
    fn discord_inline_code(self) -> Self;

    #[must_use]
    fn discord_italic(self) -> Self;

    #[must_use]
    fn discord_relative_timestamp(self) -> Self;

    #[must_use]
    fn discord_underline(self) -> Self;

    #[must_use]
    fn discord_strikethrough(self) -> Self;
}

impl MarkdownStyle for String {
    fn discord_bold(self) -> Self {
        format!("**{self}**")
    }

    fn discord_inline_code(self) -> Self {
        format!("`{self}`")
    }

    fn discord_italic(self) -> Self {
        format!("*{self}*")
    }

    fn discord_relative_timestamp(self) -> Self {
        format!("<t:{self}:R>")
    }

    fn discord_underline(self) -> Self {
        format!("__{self}__")
    }

    fn discord_strikethrough(self) -> Self {
        format!("~~{self}~~")
    }
}
