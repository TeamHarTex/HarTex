--! cached_role_select_by_guild_id (guild_id)
SELECT
    *
FROM
    "DiscordFrontendNightly".public."CachedRoles"
WHERE
    "guild_id" = :guild_id;
    