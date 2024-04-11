--! cached_emoji_upsert (id, guild_id)
INSERT INTO "DiscordFrontend"."Nightly"."CachedEmojis" ("id", "guild_id")
VALUES (:id, :guild_id)
ON CONFLICT ("id") DO UPDATE
    SET
        "guild_id" = :guild_id;
