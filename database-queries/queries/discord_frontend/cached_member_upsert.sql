--! cached_member_upsert (flags, joined_at?, nick?, user_id, guild_id, roles)
INSERT INTO "DiscordFrontend"."Nightly"."CachedMembers" ("flags", "joined_at", "nick", "user_id", "guild_id", "roles")
VALUES (:flags, :joined_at, :nick, :user_id, :guild_id, :roles)
ON CONFLICT ("user_id", "guild_id") DO UPDATE
    SET
        "flags" = :flags,
        "joined_at" = :joined_at,
        "nick" = :nick,
        "roles" = :roles;
