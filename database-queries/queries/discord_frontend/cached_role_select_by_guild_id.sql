--! cached_role_select_by_guild_id (guild_id) : (color, icon?, id, guild_id, flags, hoist, managed, mentionable, position)
SELECT
    *
FROM
    "DiscordFrontendNightly".public."CachedRoles"
WHERE
    "guild_id" = :guild_id;
    