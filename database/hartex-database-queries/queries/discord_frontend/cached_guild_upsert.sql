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

INSERT INTO
    "DiscordFrontend"."Nightly"."CachedGuilds" ("default_message_notifications", "explicit_content_filter", "features", "icon", "large", "name", "owner_id", "id", "mfa_level", "premium_subscription_count", "premium_tier", "verification_level")
VALUES (@default_message_notifications, @explicit_content_filter, @features, @icon, @large, @name, @owner_id, @id, @mfa_level, @premium_subscription_count, @premium_tier, @verification_level)
ON CONFLICT ("id") DO UPDATE
    SET
        "default_message_notifications" = @default_message_notifications,
        "explicit_content_filter" = @explicit_content_filter,
        "features" = @features,
        "icon" = @icon,
        "large" = @large,
        "mfa_level" = @mfa_level,
        "name" = @name,
        "owner_id" = @owner_id,
        "premium_subscription_count" = @premium_subscription_count,
        "premium_tier" = @premium_tier,
        "verification_level" = @verification_level;
