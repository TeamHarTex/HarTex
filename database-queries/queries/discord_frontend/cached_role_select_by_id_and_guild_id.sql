--! cached_role_select_by_id_and_guild_id : (color, icon?, id, guild_id, flags, hoist, managed, mentionable, position)
SELECT
    *
FROM
    "DiscordFrontend"."Nightly"."CachedRoles"
WHERE
    "id" = :id AND
    "guild_id" = :guild_id;
