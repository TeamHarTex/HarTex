--! cached_emoji_upsert (animated, name, id, guild_id)
INSERT INTO "DiscordFrontend"."Nightly"."CachedEmojis" ("animated", "name", "id", "guild_id")
VALUES (:animated, :name, :id, :guild_id)
ON CONFLICT ("id") DO UPDATE
    SET
        "guild_id" = :guild_id,
        "animated" = :animated,
        "name" = :name;
