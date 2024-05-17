--! cached_emoji_select_by_id : (id, guild_id, animated, name, managed)
SELECT
    *
FROM
    "DiscordFrontend"."Nightly"."CachedEmojis"
WHERE
    "id" = :id;
