CREATE SCHEMA IF NOT EXISTS "Nightly";

CREATE TABLE IF NOT EXISTS "Nightly"."CachedEmojis" (
    "id" TEXT NOT NULL PRIMARY KEY,
    "guild_id" TEXT NOT NULL,
    "animated" BOOLEAN NOT NULL,
    "name" TEXT NOT NULL,
    "managed" BOOLEAN NOT NULL,
);
