/*
 * SPDX-License-Identifier: AGPL-3.0-only
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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use proc_macro2::{Delimiter, Span, TokenStream as TokenStream2, TokenTree};
use syn::spanned::Spanned;
use syn::{AttrStyle, Data, DataEnum, DataUnion, DeriveInput, Error, Visibility};

const BOOLEAN_PARAMETERS: [&'static str; 1] = ["interaction_only"];
const LITERAL_PARAMETERS: [&'static str; 3] = ["command_type", "description", "name"];
const VALID_ATTR_PARAMETER_NAMES: [&'static str; 4] =
    ["command_type", "description", "interaction_only", "name"];

pub fn expand_command_metadata_derivation(
    input: &mut DeriveInput,
) -> Result<TokenStream2, Vec<Error>> {
    // check if item is public
    match input.vis.clone() {
        Visibility::Public(_) => {}
        visibility => {
            return Err(vec![Error::new(
                visibility.span(),
                "trait can only be derived on pub items",
            )]);
        }
    }

    // check if item is a struct
    match input.data.clone() {
        Data::Struct(_) => {}
        Data::Enum(DataEnum { enum_token, .. }) => {
            return Err(vec![Error::new(
                enum_token.span(),
                "trait can only be derived on structs",
            )]);
        }
        Data::Union(DataUnion { union_token, .. }) => {
            return Err(vec![Error::new(
                union_token.span(),
                "trait can only be derived on structs",
            )]);
        }
    }

    // check for any attributes following derive
    if input.attrs.is_empty() {
        return Err(vec![Error::new(
            Span::call_site(),
            "expected `metadata` attributes after derive",
        )]);
    }

    // split attribute vector into two
    let mut wrong_paths = input.attrs.clone();
    let correct_attrs = wrong_paths
        .drain_filter(|attr| attr.style == AttrStyle::Outer && attr.path.is_ident("metadata"))
        .collect::<Vec<_>>();

    if !wrong_paths.is_empty() {
        return Err(wrong_paths
            .into_iter()
            .map(|attr| attr.path.span())
            .map(|span| Error::new(span, "expected `metadata` attribute"))
            .collect());
    }

    let mut functions = TokenStream2::new();
    let mut previous_attr_name = String::new();
    for attr in correct_attrs {
        if attr.tokens.is_empty() {
            return Err(vec![Error::new(
                attr.path.span(),
                "unexpected end of attribute",
            )]);
        }

        let mut iter = attr.tokens.into_iter().peekable();

        // obtain the group
        let tree = iter.next().unwrap();
        let TokenTree::Group(group) = tree else {
            return Err(vec![Error::new(tree.span(), "expected token group")]);
        };

        if group.delimiter() != Delimiter::Parenthesis {
            return Err(vec![Error::new(
                group.span(),
                "expected parenthesized parameter",
            )]);
        }

        let mut group_iter = group.stream().into_iter().peekable();
        let Some(group_tree_first) = group_iter.next() else {
            return Err(vec![Error::new(group.span(), "expected parameter; found none")]);
        };

        let TokenTree::Ident(ident) = group_tree_first.clone() else {
            return Err(vec![Error::new(group_tree_first.span(), format!("expected identifier; found `{group_tree_first}`"))]);
        };

        if ident.to_string() == previous_attr_name {
            return Err(vec![Error::new(
                ident.span(),
                format!("duplicate attribute: `{ident}`"),
            )]);
        }

        if !(VALID_ATTR_PARAMETER_NAMES.contains(&ident.to_string().as_str())) {
            return Err(vec![Error::new(
                ident.span(),
                format!("unexpected parameter name: `{ident}`"),
            )]);
        }

        let Some(group_tree_next) = group_iter.next() else {
            return Err(vec![Error::new(group_tree_first.span(), "unexpected end of parameter")]);
        };

        let TokenTree::Punct(punct) = group_tree_next.clone() else {
            return Err(vec![Error::new(group_tree_next.span(), format!("expected punctuation; found `{group_tree_next}` instead"))]);
        };

        if punct.as_char() != '=' {
            return Err(vec![Error::new(
                punct.span(),
                format!("expected `=`; found `{punct}` instead"),
            )]);
        }

        let Some(group_tree_next) = group_iter.next() else {
            return Err(vec![Error::new(group_tree_next.span(), "unexpected end of parameter")]);
        };

        if LITERAL_PARAMETERS.contains(&ident.to_string().as_str()) {
            let TokenTree::Literal(literal) = group_tree_next.clone() else {
                return Err(vec![Error::new(group_tree_next.span(), "expected literal; found `{group_tree_next}`")]);
            };

            match ident.to_string().as_str() {
                "command_type" => {
                    let Ok(command_type) = literal.to_string().parse::<u8>() else {
                        return Err(vec![Error::new(literal.span(), format!("expected integer literal; found literal `{literal}`"))]);
                    };

                    if !(1..=3).contains(&command_type) {
                        return Err(vec![Error::new(
                            literal.span(),
                            format!("invalid command type: `{literal}`"),
                        )]);
                    }

                    let expanded = quote::quote! {
                        fn command_type(&self) -> u8 {
                            #group_tree_next
                        }
                    };
                    functions.extend(expanded);
                }
                "description" => {
                    let expanded = quote::quote! {
                        fn description(&self) -> String {
                            #group_tree_next
                        }
                    };
                    functions.extend(expanded);
                }
                "name" => {
                    let expanded = quote::quote! {
                        fn name(&self) -> String {
                            #group_tree_next
                        }
                    };
                    functions.extend(expanded);
                }
                _ => unreachable!(),
            }
        } else if BOOLEAN_PARAMETERS.contains(&ident.to_string().as_str()) {
            let TokenTree::Ident(ident) = group_tree_next.clone() else {
                return Err(vec![Error::new(group_tree_next.span(), format!("expected identifier; found `{group_tree_next}`"))]);
            };

            match ident.to_string().as_str() {
                "interaction_only" => {
                    let Ok(_) = ident.to_string().parse::<bool>() else {
                        return Err(vec![Error::new(ident.span(), format!("expected boolean; found `{ident}`"))]);
                    };

                    let expanded = quote::quote! {
                        fn interaction_only(&self) -> bool {
                            #group_tree_next
                        }
                    };
                    functions.extend(expanded);
                }
                _ => unreachable!(),
            }
        } else {
            unreachable!()
        };

        previous_attr_name = ident.to_string();
    }

    let mut ret = TokenStream2::new();
    let ident = input.ident.clone();
    let expanded = quote::quote! {
        impl hartex_discord_commands_core::CommandMetadata for #ident {
            #functions
        }
    };
    ret.extend(expanded);

    Ok(ret)
}
