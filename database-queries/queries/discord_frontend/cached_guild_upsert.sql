--! cached_guild_upsert
INSERT INTO
    "DiscordFrontendNightly".public."CachedGuilds" ("default_message_notifications", "explicit_content_filter", "features", "icon", "large", "name", "owner_id", "id")
VALUES (:default_message_notifications, :explicit_content_filter, :features, :icon, :large, :name, :owner_id, :id)
ON CONFLICT ("id") DO UPDATE
    SET
        "default_message_notifications" = :default_message_notifications,
        "explicit_content_filter" = :explicit_content_filter,
        "features" = :features,
        "icon" = :icon,
        "large" = :large,
        "name" = :name,
        "owner_id" = :owner_id;
