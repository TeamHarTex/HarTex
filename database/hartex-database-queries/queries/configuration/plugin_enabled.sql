--! plugin_enabled (guild_id, plugin)
SELECT
    configuration -> 'plugins' -> :plugin -> 'enabled'
FROM
    "Nightly"."GuildConfigurations"
WHERE
    guild_id = :guild_id;
