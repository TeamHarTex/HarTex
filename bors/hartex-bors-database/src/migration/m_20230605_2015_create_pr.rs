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

use crate::migration::m_20230527_2254_create_build::Build;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_query::types::Keyword;
use sea_orm_migration::sea_query::SimpleExpr;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PullRequest::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PullRequest::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PullRequest::Repository).string().not_null())
                    .col(ColumnDef::new(PullRequest::Number).integer().not_null())
                    .col(ColumnDef::new(PullRequest::Title).string().not_null())
                    .col(ColumnDef::new(PullRequest::HeadRef).string().not_null())
                    .col(ColumnDef::new(PullRequest::TryBuild).integer().null())
                    .col(ColumnDef::new(PullRequest::Url).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-pr-try-build")
                            .from(PullRequest::Table, PullRequest::TryBuild)
                            .to(Build::Table, Build::Id),
                    )
                    .col(
                        ColumnDef::new(PullRequest::CreatedAt)
                            .timestamp()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                            .not_null(),
                    )
                    .index(
                        Index::create()
                            .unique()
                            .name("unique-repo-number")
                            .col(PullRequest::Repository)
                            .col(PullRequest::Number),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PullRequest::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum PullRequest {
    Table,
    Id,
    Repository,
    Number,
    Title,
    HeadRef,
    TryBuild,
    Url,
    CreatedAt,
}
