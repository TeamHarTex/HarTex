INSERT INTO "CachedCurrentUsers" ("id", username, discriminator, avatar, flags, public_flags)
VALUES ($1, $2, $3, $4, $5, $6)
ON CONFLICT ("id") DO UPDATE
    SET username = EXCLUDED.username,
        discriminator = EXCLUDED.discriminator,
        avatar = EXCLUDED.avatar,
        flags = EXCLUDED.flags,
        public_flags = EXCLUDED.public_flags; --
