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

//! # Compile-time Reflection Types

#![allow(dead_code)]

/// Represents an enum.
#[derive(Debug, Clone)]
pub struct Enum {
    /// The name of the enum.
    pub name: String,
    /// The generic parameters of the enum.
    pub generic_params: Vec<GenericParameter>,
}

/// Represents a field.
#[derive(Debug, Clone)]
pub struct Field {
    /// The name of the field.
    pub name: String,
    /// The visibility of the field.
    pub vis: String,
    /// The type of the field.
    pub ty: String,
}

/// Represents a generic parameter.
#[derive(Debug, Clone)]
pub struct GenericParameter {
    /// The name of the generic parameter.
    pub name: String,
}

/// Represents a struct.
#[derive(Debug, Clone)]
pub struct Struct {
    /// The name of the struct.
    pub name: String,
    /// The generic parameters of the struct.
    pub generic_params: Vec<GenericParameter>,
    /// The fields of the struct.
    pub fields: Vec<Field>,
}
