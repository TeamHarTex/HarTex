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

use sea_orm_migration::prelude::*;

use crate::migration::m_20230527_2254_create_build::Build;
use crate::migration::m_20230608_2047_create_approve_build::ApproveBuild;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Workflow::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Workflow::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Workflow::ApproveBuild).integer().not_null())
                    .col(ColumnDef::new(Workflow::Build).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-workflow-approve-build")
                            .from(Workflow::Table, Workflow::ApproveBuild)
                            .to(ApproveBuild::Table, ApproveBuild::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-workflow-build")
                            .from(Workflow::Table, Workflow::Build)
                            .to(Build::Table, Build::Id),
                    )
                    .col(ColumnDef::new(Workflow::Name).string().not_null())
                    .col(ColumnDef::new(Workflow::RunId).big_unsigned().not_null())
                    .col(ColumnDef::new(Workflow::Url).string().not_null())
                    .col(ColumnDef::new(Workflow::Status).string().not_null())
                    .col(ColumnDef::new(Workflow::Type).string().not_null())
                    .col(
                        ColumnDef::new(Workflow::CreatedAt)
                            .timestamp()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                            .not_null(),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .name("unique-workflow-build-url")
                            .col(Workflow::Build)
                            .col(Workflow::Url),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Workflow::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Workflow {
    Table,
    Id,
    ApproveBuild,
    Build,
    Name,
    RunId,
    Url,
    Status,
    Type,
    CreatedAt,
}
