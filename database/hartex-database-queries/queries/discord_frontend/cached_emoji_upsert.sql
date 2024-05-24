--! cached_emoji_upsert (animated, name, id, guild_id, managed)
INSERT INTO "DiscordFrontend"."Nightly"."CachedEmojis" ("animated", "name", "id", "guild_id", "managed")
VALUES (:animated, :name, :id, :guild_id, :managed)
ON CONFLICT ("id") DO UPDATE
    SET
        "guild_id" = :guild_id,
        "animated" = :animated,
        "name" = :name,
        "managed" = :managed;
