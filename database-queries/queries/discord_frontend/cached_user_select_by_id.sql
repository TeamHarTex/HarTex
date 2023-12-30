--! cached_user_select_by_id : (id, bot)
SELECT
    *
FROM
    "DiscordFrontend"."Nightly"."CachedUsers"
WHERE
    "id" = :id;
