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

//! # Discord Markdown Utilities

use std::fmt::Display;

/// A trait for a DSL to add certain markdonw styles.
#[allow(clippy::module_name_repetitions)]
pub trait MarkdownStyle {
    /// Apply the bold style.
    #[must_use]
    fn bold(self) -> String;

    /// Apply the codeblock style.
    #[must_use]
    fn codeblock(self) -> String;

    /// Apply the footnote style.
    #[must_use]
    fn footnote(self) -> String;

    /// Apply the H1 style.
    #[must_use]
    fn h1(self) -> String;

    /// Apply the H2 style.
    #[must_use]
    fn h2(self) -> String;

    /// Apply the H3 style.
    #[must_use]
    fn h3(self) -> String;

    /// Apply the inline code style.
    #[must_use]
    fn inline_code(self) -> String;

    /// Apply the italic style.
    #[must_use]
    fn italic(self) -> String;

    /// Apply the relative timestamp style.
    #[must_use]
    fn relative_timestamp(self) -> String;

    /// Apply the underline style.
    #[must_use]
    fn underline(self) -> String;

    /// Apply the spoiler style.
    #[must_use]
    fn spoiler(self) -> String;

    /// Apply the strikethrough style.
    #[must_use]
    fn strikethrough(self) -> String;
}

impl<T> MarkdownStyle for T
where
    T: Display,
{
    fn bold(self) -> String {
        format!("**{self}**")
    }

    fn codeblock(self) -> String {
        format!("```{self}```")
    }

    fn footnote(self) -> String {
        format!("-# {self}")
    }

    fn h1(self) -> String {
        format!("# {self}")
    }

    fn h2(self) -> String {
        format!("## {self}")
    }

    fn h3(self) -> String {
        format!("### {self}")
    }

    fn inline_code(self) -> String {
        format!("`{self}`")
    }

    fn italic(self) -> String {
        format!("*{self}*")
    }

    fn relative_timestamp(self) -> String {
        format!("<t:{self}:R>")
    }

    fn underline(self) -> String {
        format!("__{self}__")
    }

    fn spoiler(self) -> String {
        format!("||{self}||")
    }

    fn strikethrough(self) -> String {
        format!("~~{self}~~")
    }
}
