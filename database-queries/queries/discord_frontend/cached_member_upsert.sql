--! cached_member_upsert (user_id, guild_id, roles)
INSERT INTO "DiscordFrontendNightly".public."CachedMembers" ("user_id", "guild_id", "roles")
VALUES (:user_id, :guild_id, :roles)
ON CONFLICT ("user_id", "guild_id") DO UPDATE
    SET
        "roles" = :roles;
