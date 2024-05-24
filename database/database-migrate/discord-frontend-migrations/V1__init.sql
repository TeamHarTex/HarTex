CREATE SCHEMA IF NOT EXISTS "Nightly";

CREATE TABLE IF NOT EXISTS "Nightly"."CachedEmojis" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "guild_id" TEXT NOT NULL,
    "animated" BOOLEAN NOT NULL,
    "name" TEXT NOT NULL,
    "managed" BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS "Nightly"."CachedGuilds" (
    "default_message_notifications" SMALLINT NOT NULL,
    "explicit_content_filter" SMALLINT NOT NULL,
    "features" TEXT[] NOT NULL,
    "icon" TEXT,
    "id" TEXT NOT NULL PRIMARY KEY,
    "large" BOOLEAN NOT NULL,
    "mfa_level" SMALLINT NOT NULL,
    "name" TEXT NOT NULL,
    "owner_id" TEXT NOT NULL,
    "premium_subscription_count" BIGINT,
    "premium_tier" SMALLINT NOT NULL,
    "verification_level" SMALLINT NOT NULL
);

CREATE TABLE IF NOT EXISTS "Nightly"."CachedMembers" (
    "flags" BIGINT NOT NULL,
    "joined_at" TIMESTAMP WITH TIME ZONE
    "nick" TEXT,
    "roles" TEXT[] NOT NULL,
    "guild_id" TEXT NOT NULL,
    "user_id" TEXT NOT NULL,
    PRIMARY KEY("guild_id", "user_id")
);

CREATE TABLE IF NOT EXISTS "Nightly"."CachedRoles" (
    "color" BIGINT NOT NULL,
    "flags" INTEGER NOT NULL,
    "guild_id" TEXT NOT NULL,
    "hoist" BOOLEAN NOT NULL,
    "icon" TEXT,
    "id" TEXT NOT NULL,
    "managed" BOOLEAN NOT NULL,
    "mentionable" BOOLEAN NOT NULL,
    "position" INTEGER NOT NULL,
    PRIMARY KEY("guild_id", "id")
);

CREATE TABLE IF NOT EXISTS "Nightly"."CachedUsers" (
    "avatar" TEXT,
    "bot" BOOLEAN NOT NULL,
    "id" TEXT NOT NULL PRIMARY KEY,
    "discriminator" TEXT NOT NULL,
    "global_name" TEXT,
    "name" TEXT NOT NULL
);
