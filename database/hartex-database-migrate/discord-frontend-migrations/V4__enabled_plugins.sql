ALTER TABLE "Nightly"."GuildConfigurations"
DROP COLUMN "plugins_management_enabled",
DROP COLUMN "plugins_utilities_enabled";

ALTER TABLE "Nightly"."GuildConfigurations"
ADD COLUMN "enabled_plugins" TEXT ARRAY NOT NULL;
