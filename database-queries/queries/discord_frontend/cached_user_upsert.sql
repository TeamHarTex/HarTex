--! cached_user_upsert (id, bot, name, discriminator, global_name?)
INSERT INTO "DiscordFrontend"."Nightly"."CachedUsers" ("id", "bot", "name", "discriminator", "global_name")
VALUES (:id, :bot, :name, :discriminator, :global_name)
ON CONFLICT ("id") DO UPDATE
    SET
        "bot" = :bot,
        "name" = :name,
        "discriminator" = :discriminator,
        "global_name" = :global_name;
