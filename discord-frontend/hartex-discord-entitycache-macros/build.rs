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
#![allow(clippy::expect_fun_call)]

use std::fs;
use std::fs::File;
use std::io;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::path::Path;
use std::str;

use proc_macro2::Ident;
use proc_macro2::Span;
use syn::Item;
use syn::ItemMod;
use syn::ItemUse;
use syn::Token;
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

    let _ = build_module_tree_from_file(
        &lib_rs_path,
        &Visibility::Public(Token![pub](Span::call_site())),
    );
}

fn build_module_tree_from_mod(_: &ItemMod) -> ModuleTree {
    todo!()
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
        Item::Mod(item_mod) => if item_mod.content.is_some() {
            module_tree.children.push(build_module_tree_from_mod(item_mod));
        } else {
            let mod_name = &item_mod.ident.to_string();
            let mod_file = path.parent().unwrap().join(format!("{mod_name}.rs"));
            let mod_dir_file = path.parent().unwrap().join(mod_name).join("mod.rs");

            if mod_file.exists() ||  mod_dir_file.exists() {
                module_tree.children.push(
                    build_module_tree_from_file(&mod_file, &item_mod.vis)
                );
            }
        }
        Item::Use(item_use) => module_tree.reexports.push(item_use.clone()),
        item => module_tree.items.push(item.clone())
    });

    module_tree
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
