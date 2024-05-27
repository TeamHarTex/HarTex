--! general_plugin_enabled (guild_id)
SELECT
    configuration -> 'plugins' -> 'general' -> 'enabled'
FROM
    "Nightly"."GuildConfigurations"
WHERE
    guild_id = :guild_id;
