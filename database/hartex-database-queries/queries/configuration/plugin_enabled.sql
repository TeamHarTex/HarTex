--! plugin_enabled (guild_id, plugin)
SELECT
    TRUE
FROM
    "Nightly"."GuildConfigurations"
WHERE
    "enabled_plugins" @> array[ :plugin ] AND
    "guild_id" = :guild_id;
