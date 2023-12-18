--! cached_member_select_by_guild_id_and_user_id : (user_id, guild_id, roles)
SELECT
    *
FROM
    "DiscordFrontendNightly".public."CachedMembers"
WHERE
    user_id = :user_id AND
    guild_id = :guild_id;
