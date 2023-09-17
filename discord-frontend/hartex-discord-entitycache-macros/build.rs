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

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
#![deny(warnings)]
#![allow(dead_code)]
#![allow(clippy::expect_fun_call)]

use std::fs;
use std::fs::File;
use std::io;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::path::Path;
use std::str;
use std::str::FromStr;

use convert_case::Case;
use convert_case::Casing;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;
use syn::ItemMod;
use syn::ItemStruct;
use syn::ItemUse;
use syn::LitStr;
use syn::Token;
use syn::UsePath;
use syn::Visibility;
use zip::ZipArchive;

#[cfg(feature = "discord_model_v_0_15_4")]
const MODEL_CRATE_VERSION: &str = "0.15.4";

struct ModuleTree {
    children: Vec<ModuleTree>,
    items: Vec<Item>,
    name: Ident,
    reexports: Vec<ItemUse>,
    visibility: Visibility,
}

pub fn main() {
    let response = reqwest::blocking::get(format!("https://github.com/twilight-rs/twilight/archive/refs/tags/twilight-model-{MODEL_CRATE_VERSION}.zip"))
        .expect(&format!("twilight-model {MODEL_CRATE_VERSION} is not found"));
    let bytes = response.bytes().expect("failed to obtain archive bytes");

    // extract the archive
    let reader = Cursor::new(bytes);
    let output_dir = Path::new("downloaded");

    extract_archive(reader, output_dir);

    let crate_dir = output_dir.join(format!(
        "twilight-twilight-model-{MODEL_CRATE_VERSION}/twilight-model"
    ));
    let lib_rs_path = crate_dir.join("src/lib.rs");

    let module_tree = build_module_tree_from_file(
        &lib_rs_path,
        &Visibility::Public(Token![pub](Span::call_site())),
    );

    let metadata_from_module_tree = generate_metadata_from_module_tree(&module_tree, false);
    let struct_metadata_map = generate_struct_metadata_map(&module_tree);

    let code = quote! {
        use std::collections::HashMap;
        use std::ops::Deref;

        #metadata_from_module_tree

        #struct_metadata_map
    };

    let generated_dir = Path::new("generated");
    fs::create_dir_all(&generated_dir).expect("failed to create directory");

    let metadata_file_path = generated_dir.join("metadata.rs");
    let mut file = File::create(metadata_file_path).expect("failed to create file");
    file.write_all(code.to_string().as_bytes())
        .expect("failed to write code to file");
}

fn build_module_tree_from_mod(item_mod: &ItemMod) -> ModuleTree {
    let mut module_tree = ModuleTree {
        children: vec![],
        items: vec![],
        name: item_mod.ident.clone(),
        reexports: vec![],
        visibility: item_mod.vis.clone(),
    };

    item_mod
        .content
        .as_ref()
        .unwrap()
        .1
        .iter()
        .for_each(|item| match item {
            Item::Mod(item_mod) => {
                if item_mod.ident == "tests" || item_mod.ident == "test" {
                    // do nothing
                } else if item_mod.content.is_some() {
                    module_tree
                        .children
                        .push(build_module_tree_from_mod(item_mod));
                } else {
                    let mod_name = &item_mod.ident.to_string();
                    let mod_file = Path::new(mod_name).with_extension("rs");
                    let mod_dir_file = Path::new(mod_name).join(mod_name).join("mod.rs");

                    if mod_file.exists() {
                        module_tree
                            .children
                            .push(build_module_tree_from_file(&mod_file, &item_mod.vis));
                    } else if mod_dir_file.exists() {
                        module_tree
                            .children
                            .push(build_module_tree_from_file(&mod_dir_file, &item_mod.vis));
                    }
                }
            }
            Item::Use(item_use) if matches!(item_use.vis, syn::Visibility::Public(_)) => {
                module_tree.reexports.push(item_use.clone())
            }
            item => module_tree.items.push(item.clone()),
        });

    module_tree
}

fn build_module_tree_from_file(path: &Path, visibility: &Visibility) -> ModuleTree {
    let content = fs::read_to_string(path)
        .expect(&format!("failed to read file: {}", path.to_string_lossy()));
    let rust_file = syn::parse_file(&content)
        .expect(&format!("failed to parse file: {}", path.to_string_lossy()));

    let module_name = if path.file_stem().unwrap() == "mod" {
        path.parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
    } else {
        path.file_stem().unwrap().to_str().unwrap()
    };

    let mut module_tree = ModuleTree {
        children: vec![],
        items: vec![],
        name: Ident::new(module_name, Span::call_site()),
        reexports: vec![],
        visibility: visibility.clone(),
    };

    rust_file.items.iter().for_each(|item| match item {
        Item::Mod(item_mod) => {
            if item_mod.ident == "tests" || item_mod.ident == "test" {
                // do nothing
            } else if item_mod.content.is_some() {
                module_tree
                    .children
                    .push(build_module_tree_from_mod(item_mod));
            } else {
                let mod_name = &item_mod.ident.to_string();
                let mod_file = path.parent().unwrap().join(format!("{mod_name}.rs"));
                let mod_dir_file = path.parent().unwrap().join(mod_name).join("mod.rs");

                if mod_file.exists() {
                    module_tree
                        .children
                        .push(build_module_tree_from_file(&mod_file, &item_mod.vis));
                } else if mod_dir_file.exists() {
                    module_tree
                        .children
                        .push(build_module_tree_from_file(&mod_dir_file, &item_mod.vis));
                }
            }
        }
        Item::Use(item_use) if matches!(item_use.vis, syn::Visibility::Public(_)) => {
            module_tree.reexports.push(item_use.clone())
        }
        item => module_tree.items.push(item.clone()),
    });

    module_tree
}

fn collect_type_to_module_path_mappings(tree: &ModuleTree, current_path: &str) -> Vec<(String, String)> {
    let mut mappings = Vec::new();

    // Get the current path
    let current_path =
        format!("{}::{}", current_path, tree.name).replace("twilight_model::lib", "twilight_model");

    // If the current item is a struct, capture its path
    for item in &tree.items {
        match item {
            Item::Struct(item_struct) => {
                let type_descriptor = item_struct.ident.to_string();
                let path_descriptor = format!("{}::{}", current_path, type_descriptor);
                mappings.push((type_descriptor, path_descriptor));
            }
            Item::Enum(item_enum) => {
                let type_descriptor = item_enum.ident.to_string();
                let path_descriptor = format!("{}::{}", current_path, type_descriptor);
                mappings.push((type_descriptor, path_descriptor));
            }
            _ => {}
        }
    }

    for item_use in &tree.reexports {
        if let syn::UseTree::Path(use_path) = &item_use.tree {
            let idents = extract_identifiers_from_use_path(&use_path);
            for last_segment in idents {
                let type_name = last_segment.to_string();
                let full_path = format!("{}::{}", current_path, type_name);
                mappings.push((type_name, full_path));
            }
        } else if let syn::UseTree::Group(use_group) = &item_use.tree {
            for inner_tree in &use_group.items {
                if let syn::UseTree::Path(use_path) = inner_tree {
                    let idents = extract_identifiers_from_use_path(&use_path);
                    for last_segment in idents {
                        let type_name = last_segment.to_string();
                        let full_path = format!("{}::{}", current_path, type_name);
                        mappings.push((type_name, full_path));
                    }
                }
            }
        }
    }

    // Recursively collect from public child modules
    for child in &tree.children {
        if !tree.name.eq("http") {
            if matches!(child.visibility, syn::Visibility::Public(_)) {
                mappings.extend(collect_type_to_module_path_mappings(child, &current_path));
            }
        }
    }

    mappings

}

fn extract_archive<R: Read + Seek>(reader: R, output_dir: &Path) {
    let mut archive = ZipArchive::new(reader).expect("failed to open zip archive");

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .expect("failed to obtain file in zip archive");

        if !file.name().starts_with(&format!(
            "twilight-twilight-model-{MODEL_CRATE_VERSION}/twilight-model"
        )) {
            continue;
        }

        let file_path = output_dir.join(file.name());

        if file.name().ends_with('/') {
            fs::create_dir_all(&file_path).expect("failed to create directory");
        } else {
            let mut output_file = File::create(&file_path).expect("failed to create file");
            io::copy(&mut file, &mut output_file).expect("failed to copy file from zip");
        }
    }
}

fn extract_identifiers_from_use_path(use_path: &UsePath) -> Vec<Ident> {
    let mut identifiers = Vec::new();

    match &use_path.tree.as_ref() {
        syn::UseTree::Name(use_name) => {
            identifiers.push(use_name.ident.clone());
        }
        syn::UseTree::Path(inner_path) => {
            identifiers.extend(extract_identifiers_from_use_path(inner_path));
        }
        syn::UseTree::Group(use_group) => {
            for inner_tree in &use_group.items {
                if let syn::UseTree::Path(inner_use_path) = inner_tree {
                    identifiers.extend(extract_identifiers_from_use_path(inner_use_path));
                }
            }
        }
        _ => {}
    }

    identifiers
}

fn generate_lazy_static_from_item_struct(item_struct: &ItemStruct) -> TokenStream {
    let ident = &item_struct.ident;
    let const_name = ident.to_string().to_case(Case::ScreamingSnake);
    let const_ident = TokenStream::from_str(&const_name).unwrap();

    let generic_params = item_struct
        .generics
        .params
        .iter()
        .map(|generic_param| {
            if let syn::GenericParam::Type(type_param) = generic_param {
                let ident = &type_param.ident;

                Some(quote! {
                    crate::reflect::GenericParameter {
                        name: stringify!(#ident).to_string()
                    }
                })
            } else {
                None
            }
        })
        .filter(|option| option.is_some())
        .map(|option| option.unwrap())
        .collect::<Vec<_>>();

    let fields = item_struct
        .fields
        .iter()
        .map(|field| {
            let ident = &field.ident;
            let vis = &field.vis;
            let ty = &field.ty;

            quote! {
                crate::reflect::Field {
                    name: stringify!(#ident).to_string(),
                    vis: stringify!(#vis).to_string(),
                    ty: stringify!(#ty).to_string(),
                }
            }
        })
        .collect::<Vec<_>>();

    quote! {
        pub static ref #const_ident: crate::reflect::Struct = crate::reflect::Struct {
            name: stringify!(#ident).to_string(),
            generic_params: vec![#(#generic_params,)*],
            fields: vec![#(#fields,)*]
        };
    }
}

fn generate_metadata_from_module_tree(tree: &ModuleTree, nest: bool) -> TokenStream {
    let name = &tree.name;
    let items = &tree.items;
    let children = tree
        .children
        .iter()
        .map(|child| generate_metadata_from_module_tree(child, true))
        .collect::<Vec<_>>();

    let structs = items
        .iter()
        .map(|item| {
            if let Item::Struct(item_struct) = item {
                Some(item_struct)
            } else {
                None
            }
        })
        .filter(|option| option.is_some())
        .map(|option| option.unwrap())
        .map(|item_struct| generate_lazy_static_from_item_struct(&item_struct))
        .collect::<Vec<_>>();

    if nest {
        quote! {
            pub mod #name {
                use lazy_static::lazy_static;

                lazy_static! {
                    #(#structs)*
                }

                #(#children)*
            }
        }
    } else {
        quote! {
            use lazy_static::lazy_static;

            lazy_static! {
                #(#structs)*
            }

            #(#children)*
        }
    }
}

fn generate_struct_metadata_map(tree: &ModuleTree) -> TokenStream {
    let reexported_paths = collect_type_to_module_path_mappings(&tree, "twilight_model");
    let paths = generate_module_path_from_tree("twilight_model", tree);
    let entries = paths
        .into_iter()
        .map(|path| {
            let path = format!("{}{}", &path[0..16], &path[21..]);
            let key_literal = LitStr::new(&path, Span::call_site());
            let path_parts: Vec<_> = path.split("::").collect();
            let struct_name = path_parts.last().unwrap();
            let reexported_path = reexported_paths
                .iter()
                .filter(|(export_struct, _path)| struct_name.eq(export_struct))
                .map(|(_, path)| path.clone())
                .next();
            let value_path_name = format!(
                "{}::{}",
                &path[16..(path.len() - (struct_name.len() + 2))],
                struct_name.to_case(Case::ScreamingSnake),
            );
            let value_path: syn::Path = syn::parse_str(&value_path_name).unwrap();

            let additional_insert = if let Some(reexported_path) = reexported_path {
                if !reexported_path.eq(&value_path_name) {
                    let key_literal = LitStr::new(&reexported_path, Span::call_site());
                    quote! {
                        map.insert(#key_literal, #value_path.deref());
                    }
                } else {
                    quote!()
                }
            } else {
                quote!()
            };

            quote! {
                map.insert(#key_literal, #value_path.deref());
                #additional_insert
            }
        })
        .collect::<Vec<_>>();
    quote! {
        fn create_struct_map() -> HashMap<&'static str, &'static crate::reflect::Struct> {
            let mut map = HashMap::new();
            #(#entries)*
            map
        }

        lazy_static::lazy_static! {
            pub(crate) static ref STRUCT_MAP: HashMap<&'static str, &'static crate::reflect::Struct> = create_struct_map();
        }
    }
}

fn generate_module_path_from_tree(base_path: &str, tree: &ModuleTree) -> Vec<String> {
    let mut paths = vec![];

    // Construct the new base path
    let new_base_path = format!("{}::{}", base_path, tree.name);

    for item in &tree.items {
        if let Item::Struct(s) = item {
            paths.push(format!("{}::{}", new_base_path, s.ident));
        }
    }

    // Recurse for child modules
    for child in &tree.children {
        paths.extend(generate_module_path_from_tree(&new_base_path, child));
    }

    paths
}
