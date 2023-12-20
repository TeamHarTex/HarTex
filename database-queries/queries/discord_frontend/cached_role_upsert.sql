--! cached_role_upsert (color, icon?, id, guild_id, flags, hoist, managed, mentionable, position)
INSERT INTO "DiscordFrontendNightly".public."CachedRoles" ("color", "icon", "id", "guild_id", "flags", "hoist", "managed", "mentionable", "position")
VALUES (:color, :icon, :id, :guild_id, :flags, :hoist, :managed, :mentionable, :position)
ON CONFLICT ("id", "guild_id") DO UPDATE
    SET
        "color" = :color,
        "icon" = :icon,
        "flags" = :flags,
        "hoist" = :hoist,
        "managed" = :managed,
        "mentionable" = :mentionable,
        "position" = :position;
