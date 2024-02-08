--! cached_user_upsert (avatar?, id, bot, name, discriminator, global_name?)
INSERT INTO "DiscordFrontend"."Nightly"."CachedUsers" ("avatar", "id", "bot", "name", "discriminator", "global_name")
VALUES (:avatar, :id, :bot, :name, :discriminator, :global_name)
ON CONFLICT ("id") DO UPDATE
    SET
        "avatar" = :avatar,
        "bot" = :bot,
        "name" = :name,
        "discriminator" = :discriminator,
        "global_name" = :global_name;
