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

use proc_macro::TokenStream;

use crate::internal::derive::DeriveAttribute;
use crate::internal::StreamParser;

pub fn expand_command_metadata_derivation(tokens: TokenStream) -> TokenStream {
    let mut ret = TokenStream::new();
    let Some(stream) = crate::internal::derive::DeriveStream::parse(tokens) else {
        return ret;
    };

    let mut functions = TokenStream::new();
    stream.attributes.into_iter().for_each(|attribute| {
        let expanded = match attribute {
            DeriveAttribute::Description(description) => proc_macro::quote! {
                fn description(&self) -> String {
                    String::from($description)
                }
            },
            DeriveAttribute::InteractionOnly(interaction_only) => proc_macro::quote! {
                fn interaction_only(&self) -> bool {
                    $interaction_only
                }
            },
            DeriveAttribute::Name(name) => proc_macro::quote! {
                fn name(&self) -> String {
                    String::from($name)
                }
            },
            DeriveAttribute::CommandType(command_type) => proc_macro::quote! {
                fn command_type(&self) -> u8 {
                    $command_type
                }
            },
        };

        functions.extend(expanded);
    });

    let Some(ident) = stream.identifier else {
        return ret;
    };
    let expanded = proc_macro::quote! {
        impl hartex_discord_commands_core::CommandMetadata for $ident {
            $functions
        }
    };
    ret.extend(expanded);

    ret
}
