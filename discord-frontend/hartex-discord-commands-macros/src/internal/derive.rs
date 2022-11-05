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

use proc_macro::{Span, TokenStream, TokenTree};

use crate::internal::StreamParser;

pub struct DeriveStream;

impl StreamParser for DeriveStream {
    fn parse(tokens: TokenStream) -> Self {
        let mut iter = tokens.into_iter();
        let Some(first) = iter.next() else {
            unreachable!()
        };

        match first {
            TokenTree::Punct(_) => (),
            _ => first
                .span()
                .error("no metadata attributes found after derive")
                .span_note(
                    Span::call_site(),
                    "metadata attributes are expected after the derive invocation",
                )
                .emit(),
        }

        DeriveStream
    }
}
