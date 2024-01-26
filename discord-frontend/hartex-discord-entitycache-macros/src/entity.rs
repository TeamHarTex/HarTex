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

use std::collections::HashMap;

use convert_case::Case;
use convert_case::Casing;
use hartex_macro_utils::bail;
use hartex_macro_utils::expect;
use hartex_macro_utils::impl_bracket_parse;
use hartex_macro_utils::impl_parse;
use itertools::Itertools;
use pluralizer::pluralize;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::TokenStreamExt;
use syn::spanned::Spanned;
use syn::Expr;
use syn::ExprArray;
use syn::ExprLit;
use syn::ItemStruct;
use syn::Lit;
use syn::LitStr;
use syn::Token;
use syn::Type;

use crate::metadata;
use crate::reflect::Field;
use crate::typeext::TypeExt;

const PRELUDE_AND_PRIMITIVES: [&str; 21] = [
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "&str",
    "bool", "char", "f32", "f64", "Option", "Box", "String", "Vec",
];

const VALID_ENTITIES: [(&str, &str); 4] = [
    ("GuildEntity", "crate::guild::GuildEntity"),
    ("MemberEntity", "crate::member::MemberEntity"),
    ("RoleEntity", "crate::role::RoleEntity"),
    ("UserEntity", "crate::user::UserEntity"),
];

impl_parse!(
#[allow(dead_code)]
#[allow(clippy::module_name_repetitions)]
pub struct EntityMacroInput where
    from_ident: Ident,
    equal1: Token![=],
    from_lit_str: LitStr,
    comma1: Token![,],
    assume_ident: Ident,
    equal7: Token![=],
    assume_lits: ExprArray,
    comma7: Token![,],
    id_ident: Ident,
    equal3: Token![=],
    id_array: ExprArray,
    comma3: Token![,],
    exclude_or_include_ident: Ident,
    equal2: Token![=],
    exclude_or_include_array: ExprArray,
    comma5: Token![,],
    extra_fields_ident: Ident,
    equal5: Token![=],
    extra_fields_array: KeyValueArray,
    comma2: Token![,],
    overrides_ident: Ident,
    equal4: Token![=],
    overrides_array: KeyValueArray,
    comma4: Token![,],
    relates_idents: Ident,
    equal6: Token![=],
    relates_array: RelatesArray,
    comma6??: Token![,],
);

impl_bracket_parse!(
#[derive(Clone)]
struct KeyValueArray where
    #[allow(dead_code)]
    bracket_token,
    elements => KeyValueArrayElement,
);

impl_parse!(
#[derive(Clone)]
struct KeyValueArrayElement where
    key: LitStr,
    colon: Token![:],
    value: LitStr,
);

impl_bracket_parse!(
#[derive(Clone)]
struct RelatesArray where
    #[allow(dead_code)]
    bracket_token,
    #[allow(dead_code)]
    elements => RelatesArrayElement,
);

impl_parse!(
#[derive(Clone)]
struct RelatesArrayElement where
    unique_or_multiple: Ident,
    name: LitStr,
    colon: Token![:],
    via: Ident,
    // FIXME: may need to generalize for more than one fields
    value: LitStr,
    r#as: Token![as],
    as_value: LitStr,
);

fn type_of(ty: &str, input: &EntityMacroInput) -> syn::Type {
    syn::parse_str(&expand_fully_qualified_type_name(
        ty,
        &input.overrides_array,
    ))
    .unwrap()
}

#[allow(clippy::module_name_repetitions)]
#[allow(clippy::too_many_lines)]
pub fn implement_entity(input: &EntityMacroInput, item_struct: &ItemStruct) -> Option<TokenStream> {
    expect!(input:
        from_ident == "from";
        assume_ident == "assume";
        id_ident == "id";
        extra_fields_ident == "extra";
        overrides_ident == "overrides"
    );

    let type_key = input.from_lit_str.value();
    let rfind_index = type_key.rfind(':').unwrap();
    let end = &type_key[rfind_index..];

    let type_metadata = metadata::STRUCT_MAP.iter().find(|(key, _)| key.ends_with(end)).map_or_else(
        || {
            (input.from_lit_str.span().unwrap())
                .error(format!("type `{type_key}` cannot be found"))
                .note(format!(
                    "the type metadata generated was for twilight-model version {}",
                    metadata::CRATE_VERSION
                ))
                .help("consider regenerating the metadata for a newer version if the type is recently added")
                .emit();
            None
        },
        |(_, v)| Some(v))?;

    let mut any_not_found = false;
    let fields = input.exclude_or_include_array.elems.iter().filter_map(|expr| {
        match expr {
            Expr::Lit(ExprLit {
                lit: Lit::Str(lit_str),
                ..
            }) => {
                if type_metadata.fields.iter().any(|field| field.name == lit_str.value()) {
                    return Some(lit_str.value())
                }
                lit_str.span().unwrap()
                    .error(format!("field `{}` cannot be found in type `{type_key}`", lit_str.value()))
                    .note(format!(
                        "the type metadata generated was for twilight-model version {}",
                        metadata::CRATE_VERSION
                    ))
                    .help("consider regenerating the metadata for a newer version if the field is recently added")
                    .emit();
                any_not_found = true;
            }
            expr => expr.span().unwrap().warning("non-string expressions are ignored").emit(),
        }
        None
    });
    let fields: Vec<_> = fields.collect();

    let id_fields = input.id_array.elems.iter().filter_map(|expr| {
        match expr {
            Expr::Lit(ExprLit {
                lit: Lit::Str(lit_str),
                ..
            }) => {
                if type_metadata.fields.iter().any(|field| field.name == lit_str.value())
                    || input.extra_fields_array.elements.iter().any(|element| element.key.value() == lit_str.value()) {
                    return Some(lit_str.value());
                }

                lit_str.span().unwrap()
                    .error(format!("field `{}` cannot be found in type `{type_key}`", lit_str.value()))
                    .note(format!(
                        "the type metadata generated was for twilight-model version {}",
                        metadata::CRATE_VERSION
                    ))
                    .help("consider regenerating the metadata for a newer version if the field is recently added")
                    .emit();
                any_not_found = true;
            }
            expr => bail(expr, "non-string expressions are ignored")?,
        }
        None
    });
    let id_fields = id_fields.collect::<Vec<_>>();

    input.relates_array.elements.iter().for_each(|element| {
        if !fields.contains(&element.value.value()) && !id_fields.contains(&element.value.value()) {
            (element.value.span().unwrap())
                .error(format!(
                    "field `{}` cannot be found in type `{type_key}`",
                    element.value.value()
                ))
                .emit();
            any_not_found = true;
        }
    });

    if any_not_found {
        return None;
    }

    let item_struct_vis = item_struct.vis.clone();
    let item_struct_name = item_struct.ident.clone();
    let maker = |field: &Field| {
        let field_name = Ident::new(&field.name, Span::call_site());
        make_field_decl_and_assignments(&field_name, &type_of(&field.ty, &input))
    };

    macro_rules! filterer {
        (@$decider:literal) => {
            |field: &Field| { // flip the condition
                if fields.contains(&field.name) ^ $decider {
                    Some(maker(field))
                } else {
                    None
                }
            }
        };
        ($decider:literal) => {{
            type_metadata.fields.iter().filter_map(filterer!(@$decider)).multiunzip()
        }};
    }
    let (mut fields_tokens, mut fields_assignments, mut field_assignments_with_necessary_casts): (
        Vec<_>,
        Vec<_>,
        Vec<_>,
    ) = match &*input.exclude_or_include_ident.to_string() {
        "exclude" => filterer!(true),
        "include" => filterer!(false),
        _ => bail(
            &input.exclude_or_include_ident,
            "expected `exclude` or `include`",
        )?,
    };

    let (
        mut field_tokens_to_append,
        mut field_assignments_to_append,
        mut field_assignments_to_append_with_necessary_casts,
    ): (Vec<_>, Vec<_>, Vec<_>) = (type_metadata.fields.iter())
        .filter_map(|field| (id_fields.iter().find(|&x| x == &field.name)).map(|_| maker(field)))
        .multiunzip();

    fields_tokens.append(&mut field_tokens_to_append);
    fields_assignments.append(&mut field_assignments_to_append);
    field_assignments_with_necessary_casts
        .append(&mut field_assignments_to_append_with_necessary_casts);

    let type_tokens = if let [first] = &id_fields[..] {
        let field = (type_metadata.fields.iter())
            .find(|field| &field.name == first)
            .unwrap();

        type_of(&field.ty, &input).to_token_stream()
    } else {
        let vec = id_fields.iter().map(|name| {
            (type_metadata.fields.iter().cloned())
                .chain(
                    (input.extra_fields_array.elements.iter()).map(|element| Field {
                        name: element.key.value(),
                        vis: "pub".to_string(),
                        ty: element.value.value(),
                    }),
                )
                .find(|field| &field.name == name)
                .unwrap()
        });
        let vec = (vec.map(|field| type_of(&field.ty, &input))).collect::<Vec<_>>();

        quote! {
            (#(#vec),*)
        }
    };

    let id_field_expr_tokens = if let [first] = &id_fields[..] {
        let ident = Ident::new(first, Span::call_site()).to_token_stream();
        quote! { self.#ident }
    } else {
        let vec = id_fields.iter().map(|name| {
            let ident = Ident::new(name, Span::call_site()).to_token_stream();
            quote! { self.#ident }
        });
        let vec = vec.collect::<Vec<_>>();

        quote! { (#(#vec),*) }
    };

    let attrs = &item_struct.attrs;
    let from_type = syn::parse_str::<Type>(&type_key).unwrap();

    let fields_for_function_decls =
        (type_metadata.fields.iter()).map(|field| (field.name.clone(), field.ty.clone()));
    let fields_for_function_decls = fields_for_function_decls.merge(
        (input.extra_fields_array.elements.iter())
            .map(|element| (element.key.value(), element.value.value())),
    );
    let fields_for_function_decls = fields_for_function_decls.collect::<HashMap<_, _>>();

    let mut function_decls = Vec::new();
    for element in &input.relates_array.elements {
        if !["multiple", "unique"].contains(&&*element.unique_or_multiple.to_string()) {
            bail(
                &element.unique_or_multiple,
                "expected either `multiple` or `unique`",
            )?;
        }

        expect!(element: via == "via");

        let hashmap = VALID_ENTITIES.into_iter().collect::<HashMap<_, _>>();

        if !hashmap.keys().any(|name| name == &&*element.name.value()) {
            bail(&element.name, "unknown entity name")?;
        }

        let entity = element.name.value();
        let cased_entity = entity.to_case(Case::Snake);
        let first: &str = cased_entity.split('_').next().unwrap();

        // FIXME: may need to generalize for multiple fields
        let (param_decl, param_name) = {
            let name = Ident::new(&*element.value.value(), Span::call_site());

            let ty = (fields_for_function_decls.get(&*element.value.value())).unwrap();
            let ty = type_of(ty, &input);

            (quote! {#name: #ty}, name)
        };
        let ret_type = syn::parse_str::<Type>(hashmap.get(&*entity).unwrap()).unwrap();

        let query_function_name = make_query_function_name(first, &*element.as_value.value());
        // FIXME: bad assumption of always calling .to_string() here (mostly just that should suffice, but...)
        let mut full_query_function_call = quote! {
            let data = hartex_database_queries::discord_frontend::queries::#query_function_name::#query_function_name().bind(client, &#param_name.to_string())
        };

        let function = match &*element.unique_or_multiple.to_string() {
            "multiple" => {
                let ident = Ident::new(&pluralize(first, 2, false), Span::call_site());

                full_query_function_call.append_all(quote! {
                    .all().await?;
                });

                quote! {
                    pub async fn #ident(&self, #param_decl) -> hartex_discord_entitycache_core::error::CacheResult<Vec<#ret_type>> {
                        let pinned = std::pin::Pin::static_ref(&hartex_discord_utils::DATABASE_POOL).await;
                        let pooled = pinned.get().await?;
                        let client = pooled.client();

                        #full_query_function_call

                        Ok(data.into_iter().map(|thing| #ret_type::from(thing)).collect())
                    }
                }
            }
            "unique" => {
                let ident = Ident::new(first, Span::call_site());

                full_query_function_call.append_all(quote! {
                    .one().await?;
                });

                quote! {
                    pub async fn #ident(&self, #param_decl) -> hartex_discord_entitycache_core::error::CacheResult<#ret_type> {
                        let pinned = std::pin::Pin::static_ref(&hartex_discord_utils::DATABASE_POOL).await;
                        let pooled = pinned.get().await?;
                        let client = pooled.client();

                        #full_query_function_call

                        Ok(#ret_type::from(data))
                    }
                }
            }
            _ => unreachable!(),
        };
        function_decls.push(quote! {#function});
    }

    let assumed_extra_impls = (input.assume_lits.elems.iter()).filter_map(|expr| match expr {
        Expr::Lit(ExprLit {
            lit: Lit::Str(lit_str),
            ..
        }) => Some(lit_str.value()),
        _ => None,
    });

    if input.extra_fields_array.elements.is_empty() {
        let extra = assumed_extra_impls
            .map(|str| {
                let ident_snake = Ident::new(&str.to_case(Case::Snake), Span::call_site());
                let ident_pascal = Ident::new(&str.to_case(Case::Pascal), Span::call_site());
                let full_ident =
                    quote! {hartex_database_queries::discord_frontend::queries::#ident_snake::#ident_pascal};

                quote! {
                    impl From<#full_ident> for #item_struct_name {
                        fn from(model: #full_ident) -> Self {
                            Self { #(#field_assignments_with_necessary_casts),* }
                        }
                    }
                }
            })
            .collect::<Vec<_>>();

        return Some(quote! {
            use tokio_postgres::GenericClient;

            #(#attrs)*
            #item_struct_vis struct #item_struct_name {
                #(#fields_tokens),*
            }
            impl #item_struct_name {
                #(#function_decls)*
            }
            #[automatically_derived]
            impl hartex_discord_entitycache_core::traits::Entity for #item_struct_name {
                type Id = #type_tokens;
                fn id(&self) -> <Self as hartex_discord_entitycache_core::traits::Entity>::Id {
                    #id_field_expr_tokens
                }
            }
            impl From<#from_type> for #item_struct_name {
                fn from(model: #from_type) -> Self {
                    Self { #(#fields_assignments),* }
                }
            }
            #(#extra)*
        });
    }

    let fields = input.extra_fields_array.elements.iter().map(|element| {
        let ident = Ident::new(&element.key.value(), Span::call_site());
        make_field_decl_and_assignments(&ident, &type_of(&element.value.value(), &input))
    });
    let (extra_fields_tokens, _, extra_fields_assignment_tokens_with_necessary_casts): (
        Vec<_>,
        Vec<_>,
        Vec<_>,
    ) = fields.multiunzip();
    let (extra_fields_tokens2, extra_type_tokens): (Vec<_>, Vec<_>) =
        (input.extra_fields_array.elements.iter())
            .map(|element| {
                let ident = Ident::new(&element.key.value(), Span::call_site());
                let type_token = &type_of(&element.value.value(), &input);

                (quote! {#ident}, quote! {#type_token})
            })
            .multiunzip();
    let extra = assumed_extra_impls
        .map(|str| {
            let ident_snake = Ident::new(&str.to_case(Case::Snake), Span::call_site());
            let ident_pascal = Ident::new(&str.to_case(Case::Pascal), Span::call_site());
            let full_ident =
                quote! {hartex_database_queries::discord_frontend::queries::#ident_snake::#ident_pascal};

            quote! {
                impl From<#full_ident> for #item_struct_name {
                    fn from(model: #full_ident) -> Self {
                        Self { #(#field_assignments_with_necessary_casts),*, #(#extra_fields_assignment_tokens_with_necessary_casts),* }
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    Some(quote! {
        use tokio_postgres::GenericClient;

        #(#attrs)*
        #item_struct_vis struct #item_struct_name {
            #(#fields_tokens),*,
            #(#extra_fields_tokens),*
        }
        impl #item_struct_name {
            #(#function_decls)*
        }
        #[automatically_derived]
        impl hartex_discord_entitycache_core::traits::Entity for #item_struct_name {
            type Id = #type_tokens;
            fn id(&self) -> <Self as hartex_discord_entitycache_core::traits::Entity>::Id {
                #id_field_expr_tokens
            }
        }
        impl From<(#(#extra_type_tokens),*, #from_type)> for #item_struct_name {
            fn from((#(#extra_fields_tokens2),*, model): (#(#extra_type_tokens),*, #from_type)) -> Self {
                Self { #(#fields_assignments),*, #(#extra_fields_tokens2),* }
            }
        }
        #(#extra)*
    })
}

fn expand_fully_qualified_type_name(to_expand: &str, overrides_array: &KeyValueArray) -> String {
    let to_expand = to_expand.replace(' ', "");

    let open_angle_brackets = to_expand.find('<');
    let close_angle_brackets = to_expand.rfind('>');

    if let Some((left, right)) = open_angle_brackets.zip(close_angle_brackets) {
        return format!(
            "{}<{}>",
            expand_fully_qualified_type_name(&to_expand[0..left], overrides_array),
            expand_fully_qualified_type_name(&to_expand[left + 1..right], overrides_array)
        );
    }
    if PRELUDE_AND_PRIMITIVES.contains(&&*to_expand) {
        return to_expand;
    }

    if let Some(element) =
        (overrides_array.elements.iter()).find(|elm| elm.key.value() == to_expand)
    {
        return element.value.value();
    }

    let finder = |key: &&&str| key[key.rfind(':').unwrap() + 1..] == to_expand;

    let fully_qualified = (metadata::ENUM_MAP.keys().find(finder))
        .or_else(|| metadata::STRUCT_MAP.keys().find(finder));

    fully_qualified.map_or(to_expand, ToString::to_string)
}

fn make_field_decl_and_assignments(
    field_name: &Ident,
    field_type: &Type,
) -> (TokenStream, TokenStream, TokenStream) {
    // Field name special case
    if field_name == "discriminator" {
        return (
            quote! {pub #field_name: #field_type},
            quote! {#field_name: model.#field_name},
            quote! {#field_name: model.#field_name.parse().unwrap()},
        );
    }

    // Field type special case
    if field_type.is_enum("DefaultMessageNotificationLevel")
        || field_type.is_enum("ExplicitContentFilter")
        || field_type.is_enum("MfaLevel")
        || field_type.is_enum("PremiumTier")
        || field_type.is_enum("VerificationLevel")
    {
        (
            quote! {pub #field_name: #field_type},
            quote! {#field_name: model.#field_name},
            quote! {#field_name: #field_type::from(model.#field_name as u8)},
        )
    } else if field_type.is_id() {
        (
            quote! {pub #field_name: #field_type},
            quote! {#field_name: model.#field_name},
            quote! {#field_name: std::str::FromStr::from_str(&model.#field_name).unwrap()},
        )
    } else if field_type.is_option_of("ImageHash") {
        (
            quote! {pub #field_name: #field_type},
            quote! {#field_name: model.#field_name},
            quote! {#field_name: model.#field_name.as_deref().map(|str| std::str::FromStr::from_str(str).unwrap())},
        )
    } else if field_type.is_option_of("u64") {
        (
            quote! {pub #field_name: #field_type},
            quote! {#field_name: model.#field_name},
            quote! {#field_name: model.#field_name.map(|i| i as u64)},
        )
    } else if field_type.is_vec_of("GuildFeature") {
        (
            quote! {pub #field_name: #field_type},
            quote! {#field_name: model.#field_name},
            quote! {#field_name: model.#field_name.iter().cloned().map(From::from).collect()},
        )
    } else if field_type.is_vec_of("Id") {
        (
            quote! {pub #field_name: #field_type},
            quote! {#field_name: model.#field_name},
            quote! {#field_name: model.#field_name.iter().map(|str| std::str::FromStr::from_str(str).unwrap()).collect()},
        )
    } else {
        (
            quote! {pub #field_name: #field_type},
            quote! {#field_name: model.#field_name},
            quote! {#field_name: model.#field_name},
        )
    }
}

// FIXME: may need to generalize for multiple fields
fn make_query_function_name(target_entity: &str, by_field: &str) -> Ident {
    let name = format!(
        "cached_{}_select_by_{}",
        target_entity.to_lowercase(),
        by_field.to_lowercase()
    );

    Ident::new(&name, Span::call_site())
}
