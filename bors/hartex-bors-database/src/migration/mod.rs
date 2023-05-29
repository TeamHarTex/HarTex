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

use sea_orm_migration::async_trait::async_trait;
use sea_orm_migration::MigrationTrait;
use sea_orm_migration::MigratorTrait;

mod m_20230527_2254_create_build;
mod m_20230527_2258_create_pr;
mod m_20230529_2203_create_workflow;

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m_20230527_2254_create_build::Migration),
            Box::new(m_20230527_2258_create_pr::Migration),
            Box::new(m_20230529_2203_create_workflow::Migration),
        ]
    }
}
