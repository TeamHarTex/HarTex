CREATE TABLE IF NOT EXISTS "Nightly"."GuildConfigurations" (
    "guild_id" TEXT NOT NULL PRIMARY KEY,
    "dashboard_admins" TEXT ARRAY NOT NULL,
    "dashboard_editors" TEXT ARRAY NOT NULL,
    "dashboard_viewers" TEXT ARRAY NOT NULL,
    "appearance_nickname" TEXT NOT NULL,
    "appearance_colour" BIGINT NOT NULL,
    "enabled_plugins" TEXT ARRAY NOT NULL
);
