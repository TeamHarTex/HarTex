--! cached_guild_select_by_id : (default_message_notifications, explicit_content_filter, features, icon?, large, name, owner_id, id)
SELECT 
    *
FROM
    "DiscordFrontend"."Nightly"."CachedGuilds"
WHERE
    "id" = :id;
