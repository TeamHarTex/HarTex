--! cached_user_select_by_id : (avatar?, id, bot, name, discriminator, global_name?)
SELECT
    *
FROM
    "DiscordFrontend"."Nightly"."CachedUsers"
WHERE
    "id" = :id;
