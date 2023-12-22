--! cached_member_select_by_guild_id (guild_id)
SELECT
    *
FROM
    "DiscordFrontendNightly".public."CachedMembers"
WHERE
    "guild_id" = :guild_id;
