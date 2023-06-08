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

use sea_orm::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "pull_request")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub repository: String,
    pub number: i32,
    pub assignee: String,
    pub approved_by: Option<String>,
    pub title: String,
    pub head_ref: String,
    pub approve_build: Option<i32>,
    pub try_build: Option<i32>,
    pub url: String,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::approve_build::Entity",
        from = "Column::ApproveBuild",
        to = "super::approve_build::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    ApproveBuild,
    #[sea_orm(
        belongs_to = "super::build::Entity",
        from = "Column::TryBuild",
        to = "super::build::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Build,
}

impl Related<super::approve_build::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ApproveBuild.def()
    }
}

impl Related<super::build::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Build.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
