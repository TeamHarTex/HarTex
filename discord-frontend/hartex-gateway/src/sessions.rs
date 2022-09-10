/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use std::collections::HashMap;

use hartex_core::discord::gateway::shard::ResumeSession;
use hartex_core::log;
use sqlx::postgres::PgPool;
use sqlx::{Executor, Row};

pub async fn get_sessions() -> hartex_eyre::Result<HashMap<u64, ResumeSession>> {
    log::trace!("connecting to session database");
    let credentials = std::env::var("POSTGRES_NIGHTLY_CACHE_DATABASE_CREDENTIALS")?;
    let connection = PgPool::connect(&credentials).await?;

    log::trace!("retrieving sessions");
    let output = connection
        .fetch_all(r#"SELECT * FROM "CachedSessions""#)
        .await?;

    let mut hashmap = HashMap::new();
    for row in output {
        let shard_id = row.get::<String, &str>("shard_id").parse::<u64>()?;
        let resume_url = row.get::<Option<String>, &str>("resume_url");
        let session_id = row.get::<String, &str>("session_id");
        let sequence = row.get::<String, &str>("sequence").parse::<u64>()?;

        hashmap.insert(
            shard_id,
            ResumeSession {
                resume_url,
                session_id,
                sequence,
            },
        );
    }

    Ok(hashmap)
}

pub async fn set_sessions(sessions: HashMap<u64, ResumeSession>) -> hartex_eyre::Result<()> {
    log::trace!("connecting to session database");
    let credentials = std::env::var("POSTGRES_NIGHTLY_CACHE_DATABASE_CREDENTIALS")?;
    let connection = PgPool::connect(&*credentials).await?;

    log::trace!("clearing previous sessions");
    connection
        .execute(r#"DELETE FROM "CachedSessions""#)
        .await?;

    log::trace!("loading new sessions");
    for (shard_id, session) in sessions {
        connection.execute(
            sqlx::query(r#"INSERT INTO "CachedSessions" (shard_id, resume_url, session_id, "sequence") VALUES ($1, $2, $3, $4)"#)
                .bind(shard_id.to_string())
                .bind(session.resume_url)
                .bind(session.session_id)
                .bind(session.sequence.to_string())
        ).await?;
    }

    Ok(())
}
