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

#![feature(proc_macro_diagnostic)]

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use fluent_bundle::FluentResource;
use fluent_syntax::ast::Entry;
use fluent_syntax::ast::Expression;
use fluent_syntax::ast::InlineExpression;
use fluent_syntax::ast::PatternElement;
use hartex_localization_loader::env::base_path;
use hartex_localization_loader::load_resources;
use proc_macro::Span;
use proc_macro::TokenStream;
use proc_macro2::Span as Span2;
use quote::quote;
use syn::LitStr;

struct LocalizationNode<'a> {
    category: &'a str,
    name: &'a str,
    variables: HashSet<&'a str>,
    dependencies: HashSet<&'a str>,
    term: bool,
}

impl<'a> LocalizationNode<'a> {
    pub fn new(category: &'a str, name: &'a str, term: bool) -> Self {
        Self {
            category,
            name,
            variables: HashSet::new(),
            dependencies: HashSet::new(),
            term,
        }
    }
}

#[proc_macro]
pub fn generate_bindings(_: TokenStream) -> TokenStream {
    let mut base_dir = base_path();
    base_dir.push("en-GB"); // todo: may not want to assume en-GB as default?

    let Ok(resources) = load_resources(base_dir.clone()) else {
        Span::call_site()
            .error(format!(
                "failed to load localization resources from folder: {}",
                base_dir.to_string_lossy(),
            ))
            .emit();
        return TokenStream::new();
    };

    let mut nodes = resources
        .iter()
        .flat_map(|resource| generate_nodes_for_resource(&resource.name, &resource.resource))
        .map(|node| (node.name.to_string(), node))
        .collect::<HashMap<_, _>>();

    let messages = nodes
        .iter()
        .filter(|(_, node)| !node.term)
        .map(|(name, _)| LitStr::new(name.as_str(), Span2::call_site()))
        .collect::<Vec<_>>();
    let message_count = messages.len();

    let terms = nodes
        .iter()
        .filter(|(_, node)| node.term)
        .map(|(name, _)| LitStr::new(name.as_str(), Span2::call_site()))
        .collect::<Vec<_>>();
    let term_count = terms.len();

    loop {
        let Some(dependency_name) = nodes
            .iter()
            .filter_map(|(_, node)| {
                node.dependencies
                    .iter()
                    .next()
                    .map(|dependency| dependency.to_string())
            })
            .next()
        else {
            break;
        };

        let Some((variables, dependencies)) = nodes
            .get(dependency_name.as_str())
            .map(|node| (node.variables.clone(), node.dependencies.clone()))
        else {
            panic!(
                "encountered a dependency on localization node `{dependency_name}` but no such node was loaded"
            );
        };

        for (name, node) in nodes
            .iter_mut()
            .filter(|(_, node)| node.dependencies.contains(dependency_name.as_str()))
        {
            if name.as_str() == dependency_name.as_str() {
                panic!("Cyclic localization loop detected at node {name}!");
            }

            node.dependencies.remove(dependency_name.as_str());
            node.variables.extend(variables.iter());
            node.dependencies.extend(dependencies.iter());
        }
    }

    let _ = quote! {
        pub const MESSAGES: [&str; #message_count] = [#(#messages,)*];
        pub const TERMS: [&str; #term_count] = [#(#terms,)*];

        pub struct Localizer<'a> {
            localizations: &'a hartex_localization_loader::LocalizationBundleHolder,
            language: &'a str,
        }

        impl<'a> Localizer<'a> {
            pub fn new(holder: &'a hartex_localization_loader::LocalizationBundleHolder, language: &'a str) -> Localizer<'a> {
                Self {
                    localizations: holder,
                    language,
                }
            }
        }
    };

    TokenStream::new()
}

fn generate_nodes_for_resource<'a>(
    parent: &'a str,
    resource: &'a Arc<FluentResource>,
) -> Vec<LocalizationNode<'a>> {
    let mut nodes = Vec::new();

    for entry in resource.entries() {
        let (name, pattern, term) = match entry {
            Entry::Message(message) => {
                let Some(pattern) = &message.value else {
                    continue;
                };
                (message.id.name, pattern, false)
            }
            Entry::Term(term) => (term.id.name, &term.value, true),
            _ => continue,
        };

        let mut node = LocalizationNode::new(parent, name, term);
        process_pattern_elements(&pattern.elements, &mut node);
        nodes.push(node);
    }

    nodes
}

fn process_expression<'a>(expression: &'a Expression<&'a str>, node: &mut LocalizationNode<'a>) {
    match expression {
        Expression::Inline(expression) => process_inline_expression(expression, node),
        Expression::Select { selector, variants } => {
            process_inline_expression(selector, node);
            for variant in variants {
                process_pattern_elements(&variant.value.elements, node);
            }
        }
    }
}

fn process_inline_expression<'a>(
    expression: &'a InlineExpression<&'a str>,
    node: &mut LocalizationNode<'a>,
) {
    match expression {
        InlineExpression::FunctionReference { .. } => unimplemented!(),
        InlineExpression::MessageReference { id, .. } => {
            node.dependencies.insert(id.name);
        }
        InlineExpression::TermReference { id, .. } => {
            node.dependencies.insert(id.name);
        }
        InlineExpression::VariableReference { id } => {
            node.variables.insert(id.name);
        }
        InlineExpression::Placeable { expression } => process_expression(expression, node),
        InlineExpression::StringLiteral { .. } | InlineExpression::NumberLiteral { .. } => (),
    };
}

fn process_pattern_elements<'a>(
    elements: &'a Vec<PatternElement<&'a str>>,
    node: &mut LocalizationNode<'a>,
) {
    for element in elements {
        match element {
            PatternElement::Placeable { expression } => process_expression(expression, node),
            PatternElement::TextElement { .. } => (),
        }
    }
}
