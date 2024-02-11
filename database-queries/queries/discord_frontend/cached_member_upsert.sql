--! cached_member_upsert (joined_at?, nick?, user_id, guild_id, roles)
INSERT INTO "DiscordFrontend"."Nightly"."CachedMembers" ("joined_at", "nick", "user_id", "guild_id", "roles")
VALUES (:joined_at, :nick, :user_id, :guild_id, :roles)
ON CONFLICT ("user_id", "guild_id") DO UPDATE
    SET
        "joined_at" = :joined_at,
        "nick" = :nick,
        "roles" = :roles;
