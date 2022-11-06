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

use proc_macro::{Delimiter, Span, TokenStream, TokenTree};

use crate::internal::StreamParser;

pub struct DeriveStream;

impl StreamParser for DeriveStream {
    fn parse(tokens: TokenStream) -> Self {
        eprintln!("{:?}", &tokens);

        let mut iter = tokens.into_iter().peekable();
        while let Some(first) = iter.next() && first.to_string() != String::from("pub") {
            // look for the beginning of an attribute
            //
            // #[metadata(name = "name")]
            // ^
            match first {
                TokenTree::Punct(punct) if punct.as_char() == '#' => {
                    // look for a token group of an attribute after the '#' symbol, delimited by square
                    // brackets: []
                    //
                    // #[metadata(name = "name")]
                    //  ^-----------------------^
                    match iter.peek().unwrap() {
                        TokenTree::Group(group) if group.delimiter() == Delimiter::Bracket => {
                            let mut group_tokens = group.stream().into_iter();
                            let group_first = group_tokens.next().unwrap();

                            // check if the attribute name is exactly equal to "metadata"
                            //
                            // #[metadata(name = "name)]
                            //   --------
                            match group_first.clone() {
                                TokenTree::Ident(ident)
                                if ident.to_string() == String::from("metadata") =>
                                    {
                                        let group_next_option = group_tokens.next();
                                        if group_next_option.is_some() {
                                            // look for parenthesized parameters in attribute
                                            //
                                            // #[metadata(name = "name")]
                                            //           ^------------^
                                            let group_next = group_next_option.unwrap();
                                            match group_next.clone() {
                                                TokenTree::Group(group) if group.delimiter() == Delimiter::Parenthesis => {
                                                    eprintln!("{:?}", group);
                                                },
                                                _ => group_next
                                                    .span()
                                                    .error("expected parenthesized parameters")
                                                    .emit(),
                                            }
                                        }
                                        else {
                                            group_first
                                                .span()
                                                .error("unexpected end of attribute")
                                                .emit();
                                        }
                                    }
                                _ => group_first
                                    .span()
                                    .error(format!(
                                        "expected metadata attribute; found {} attribute instead",
                                        group_first.to_string()
                                    ))
                                    .emit(),
                            }

                            iter.next();
                        }
                        _ => unreachable!(),
                    }
                }
                _ => first
                    .span()
                    .error("no metadata attributes found after derive")
                    .span_note(
                        Span::call_site(),
                        "metadata attributes are expected after the derive invocation",
                    )
                    .emit(),
            }
        }

        DeriveStream
    }
}
