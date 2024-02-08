--! cached_member_upsert (nick?, user_id, guild_id, roles)
INSERT INTO "DiscordFrontend"."Nightly"."CachedMembers" ("nick", "user_id", "guild_id", "roles")
VALUES (:nick, :user_id, :guild_id, :roles)
ON CONFLICT ("user_id", "guild_id") DO UPDATE
    SET
        "nick" = :nick,
        "roles" = :roles;
