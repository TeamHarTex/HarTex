ALTER TABLE "Nightly"."GuildConfigurations"
DROP COLUMN "configuration";

ALTER TABLE "Nightly"."GuildConfigurations"
ADD COLUMN "dashboard_admins" TEXT ARRAY NOT NULL,
ADD COLUMN "dashboard_editors" TEXT ARRAY NOT NULL,
ADD COLUMN "dashboard_viewers" TEXT ARRAY NOT NULL,
ADD COLUMN "appearance_nickname" TEXT NOT NULL,
ADD COLUMN "appearance_colour" BIGINT NOT NULL,
ADD COLUMN "plugins_management_enabled" BOOLEAN NOT NULL,
ADD COLUMN "plugins_utilities_enabled" BOOLEAN NOT NULL;
