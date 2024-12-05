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

INSERT INTO "DiscordFrontend"."Nightly"."CachedMembers" ("flags", "joined_at", "nick", "user_id", "guild_id", "roles")
VALUES (@flags, @joined_at, @nick, @user_id, @guild_id, @roles)
ON CONFLICT ("user_id", "guild_id") DO UPDATE
    SET
        "flags" = @flags,
        "joined_at" = @joined_at,
        "nick" = @nick,
        "roles" = @roles;
