--! plugin_enabled (guild_id, plugin)
SELECT
    COALESCE(MAX(configuration -> 'plugins' -> :plugin ->> 'enabled'), 'false')
FROM
    "Nightly"."GuildConfigurations"
WHERE
    guild_id = :guild_id;
