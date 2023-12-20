--! cached_user_select_by_id : (id, bot)
SELECT
    *
FROM
    "DiscordFrontendNightly".public."CachedUsers"
WHERE
    "id" = :id;
