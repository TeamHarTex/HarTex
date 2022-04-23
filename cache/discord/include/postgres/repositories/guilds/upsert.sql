INSERT INTO "CachedGuild" ("id")
VALUES ($1)
ON CONFLICT ("id") DO NOTHING; --
