--! cached_emoji_select_by_guild_id : (id, guild_id, animated, name)
SELECT
    *
FROM
    "DiscordFrontend"."Nightly"."CachedEmojis"
WHERE
    "guild_id" = :guild_id;
