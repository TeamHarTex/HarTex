--! plugin_enabled (guild_id, plugin)
SELECT
    "plugins_" + :plugin + "_enabled"
FROM
    "Nightly"."GuildConfigurations"
WHERE
    guild_id = :guild_id;
