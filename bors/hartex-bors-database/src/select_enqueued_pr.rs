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

use hartex_eyre::eyre::Report;
use sea_orm::sea_query::Alias;
use sea_orm::sea_query::IntoIden;
use sea_orm::sea_query::SelectExpr;
use sea_orm::sea_query::SelectStatement;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::FromQueryResult;
use sea_orm::Iden;
use sea_orm::JoinType;
use sea_orm::QueryFilter;
use sea_orm::QueryResult;
use sea_orm::QuerySelect;
use sea_orm::QueryTrait;
use sea_orm::RelationTrait;
use sea_orm::Select;
use serde::Serialize;

use crate::entity::approve_build;
use crate::entity::build;
use crate::entity::enqueued_pull_request;
use crate::entity::pull_request;

pub(crate) struct SelectEnqueuedPullRequest;

impl SelectEnqueuedPullRequest {

    pub async fn exec_with_repo_many(
        connection: &DatabaseConnection,
        repository: String,
    ) -> hartex_eyre::Result<
        Vec<(
            enqueued_pull_request::Model,
            pull_request::Model,
            Option<approve_build::Model>,
            Option<build::Model>,
        )>,
    > {
        let mut select = enqueued_pull_request::Entity::find()
            .select_only()
            .filter(pull_request::Column::Repository.eq(repository));


        add_columns_with_prefix::<_, enqueued_pull_request::Entity>(&mut select, "enqueued");
        add_columns_with_prefix::<_, pull_request::Entity>(&mut select, "pull_request");
        add_columns_with_prefix::<_, approve_build::Entity>(&mut select, "approve_build");
        add_columns_with_prefix::<_, build::Entity>(&mut select, "build");

        let result = execute_query_many(&mut select, connection).await?;

        Ok(result
            .iter()
            .map(|response| {
                (
                    response.enqueued_pull_request.clone(),
                    response.pull_request.clone(),
                    response.approve_build.clone(),
                    response.build.clone(),
                )
            })
            .collect())
    }
}

#[derive(Debug, Serialize)]
struct Response {
    pub enqueued_pull_request: enqueued_pull_request::Model,
    pub pull_request: pull_request::Model,
    pub approve_build: Option<approve_build::Model>,
    pub build: Option<build::Model>,
}

impl FromQueryResult for Response {
    fn from_query_result(result: &QueryResult, _pre: &str) -> Result<Self, sea_orm::DbErr> {
        let enqueued_pull_request = enqueued_pull_request::Model::from_query_result(result, "enqueued")?;
        let pull_request = pull_request::Model::from_query_result(result, "pull_request")?;
        let approve_build = approve_build::Model::from_query_result(result, "approve_build").ok();
        let build = build::Model::from_query_result(result, "build").ok();

        Ok(Self {
            enqueued_pull_request,
            pull_request,
            approve_build,
            build,
        })
    }
}

fn add_columns_with_prefix<S: QueryTrait<QueryStatement = SelectStatement>, T: EntityTrait>(
    select: &mut S,
    prefix: &'static str,
) {
    for column in <T::Column as sea_orm::entity::Iterable>::iter() {
        let alias = format!("{}{}", prefix, column.to_string());
        select.query().expr(SelectExpr {
            expr: column.select_as(column.into_expr()),
            alias: Some(Alias::new(&alias).into_iden()),
            window: None,
        });
    }
}

async fn execute_query_many(
    select: &mut Select<enqueued_pull_request::Entity>,
    connection: &DatabaseConnection,
) -> hartex_eyre::Result<Vec<Response>> {
    select
        .clone()
        .join(
            JoinType::LeftJoin,
            enqueued_pull_request::Relation::PullRequest.def(),
        )
        .join(JoinType::LeftJoin, pull_request::Relation::ApproveBuild.def())
        .join(JoinType::LeftJoin, pull_request::Relation::Build.def())
        .into_model::<Response>()
        .all(connection)
        .await
        .map_err(Report::new)
}
