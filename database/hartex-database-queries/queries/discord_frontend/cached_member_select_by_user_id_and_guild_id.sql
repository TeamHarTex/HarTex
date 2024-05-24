--! cached_member_select_by_user_id_and_guild_id : (flags, joined_at?, nick?, user_id, guild_id, roles)
SELECT
    *
FROM
    "DiscordFrontend"."Nightly"."CachedMembers"
WHERE
    user_id = :user_id AND
    guild_id = :guild_id;
