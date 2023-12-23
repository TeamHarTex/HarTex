--! cached_member_select_by_guild_id (guild_id) : (user_id, guild_id, roles)
SELECT
    *
FROM
    "DiscordFrontendNightly".public."CachedMembers"
WHERE
    "guild_id" = :guild_id;
