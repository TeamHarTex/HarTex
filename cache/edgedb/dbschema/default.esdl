module default {
    scalar type UnsignedBigInt extending bigint {
        constraint min_value(0);
    }

    type DiscordAttachment {
        property content_type -> str;
        property ephemeral -> bool;
        property filename -> str;
        property height -> UnsignedBigInt;
        required property proxy_url -> str;
        required property size -> UnsignedBigInt;
        required property snowflake_id -> UnsignedBigInt;
        required property url -> str;
        property width -> UnsignedBigInt;
    }
}
