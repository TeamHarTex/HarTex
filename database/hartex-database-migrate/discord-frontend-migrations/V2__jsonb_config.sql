CREATE TABLE IF NOT EXISTS "Nightly"."GuildConfigurations" (
    "guild_id" TEXT NOT NULL PRIMARY KEY,
    "configuration" JSONB NOT NULL
);
