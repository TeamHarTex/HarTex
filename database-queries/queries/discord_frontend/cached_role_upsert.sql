--! cached_role_upsert (icon?, id, guild_id, flags, hoist, managed, mentionable, position)
INSERT INTO "DiscordFrontendNightly".public."CachedRoles" ("icon", "id", "guild_id", "flags", "hoist", "managed", "mentionable", "position")
VALUES (:icon, :id, :guild_id, :flags, :hoist, :managed, :mentionable, :position)
ON CONFLICT ("id", "guild_id") DO UPDATE
    SET
        "icon" = :icon,
        "flags" = :flags,
        "hoist" = :hoist,
        "managed" = :managed,
        "mentionable" = :mentionable,
        "position" = :position;
