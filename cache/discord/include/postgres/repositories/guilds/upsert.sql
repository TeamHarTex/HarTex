INSERT INTO "CachedGuilds" ("id")
VALUES ($1)
ON CONFLICT ("id") DO NOTHING; --
