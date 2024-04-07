--! cached_guild_select_by_id : (default_message_notifications, emojis, explicit_content_filter, features, icon?, large, name, owner_id, id, mfa_level, premium_subscription_count?, premium_tier, verification_level)
SELECT
    *
FROM
    "DiscordFrontend"."Nightly"."CachedGuilds"
WHERE
    "id" = :id;
