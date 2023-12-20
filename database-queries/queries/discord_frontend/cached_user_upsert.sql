--! cached_user_upsert (id, bot)
INSERT INTO "DiscordFrontendNightly".public."CachedUsers" (id, bot)
VALUES (:id, :bot)
ON CONFLICT ("id") DO UPDATE
    SET
        "bot" = :bot;
