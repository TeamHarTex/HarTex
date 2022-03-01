INSERT INTO "CachedCurrentUsers" ("id", username, discriminator, avatar, bot, "system", mfa_enabled, banner, accent_colour, locale, verified, email, flags, premium_type, public_flags)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
ON CONFLICT ("id") DO UPDATE
    SET username = EXCLUDED.username,
        discriminator = EXCLUDED.discriminator,
        avatar = EXCLUDED.avatar,
        bot = EXCLUDED.bot,
        "system" = EXCLUDED."system",
        mfa_enabled = EXCLUDED.mfa_enabled,
        banner = EXCLUDED.banner,
        accent_colour = EXCLUDED.accent_colour,
        locale = EXCLUDED.locale,
        verified = EXCLUDED.verified,
        email = EXCLUDED.email,
        flags = EXCLUDED.flags,
        premium_type = EXCLUDED.premium_type,
        public_flags = EXCLUDED.public_flags;
