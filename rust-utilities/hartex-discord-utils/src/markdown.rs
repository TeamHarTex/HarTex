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

pub trait MarkdownStyle {
    fn discord_bold(self) -> Self;

    fn discord_inline_code(self) -> Self;

    fn discord_italic(self) -> Self;

    fn discord_relative_timestamp(self) -> Self;

    fn discord_underline(self) -> Self;

    fn discord_strikethrough(self) -> Self;
}

impl MarkdownStyle for String {
    fn discord_bold(self) -> Self {
        format!("**{}**", self)
    }

    fn discord_inline_code(self) -> Self {
        format!("`{}`", self)
    }

    fn discord_italic(self) -> Self {
        format!("*{}*", self)
    }

    fn discord_relative_timestamp(self) -> Self {
        format!("<t:{}:R>", self)
    }

    fn discord_underline(self) -> Self {
        format!("__{}__", self)
    }

    fn discord_strikethrough(self) -> Self {
        format!("~~{}~~", self)
    }
}
