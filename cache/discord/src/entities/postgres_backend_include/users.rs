use std::str::FromStr;

use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};

impl<'r> FromRow<'r, PgRow> for CachedCurrentUser {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let id_string = row.try_get::<String, &str>("id")?;
        let id = Id::new_checked(id_string.parse::<u64>().unwrap()).unwrap();
        let username = row.try_get::<String, &str>("username")?;
        let discriminator = row.try_get::<String, &str>("discriminator")?;
        let avatar_string = row.try_get::<Option<String>, &str>("avatar")?;
        let avatar = avatar_string.map(|string| ImageHash::from_str(&string).unwrap());
        let bot = row.try_get::<bool, &str>("bot")?;
        let system = row.try_get::<Option<bool>, &str>("system")?;
        let mfa_enabled = row.try_get::<bool, &str>("mfa_enabled")?;
        let banner_string = row.try_get::<Option<String>, &str>("banner")?;
        let banner = banner_string.map(|string| ImageHash::from_str(&string).unwrap());
        let accent_colour_string = row.try_get::<Option<String>, &str>("accent_colour")?;
        let accent_colour = accent_colour_string.map(|string| string.parse::<u64>().unwrap());
        let locale = row.try_get::<Option<String>, &str>("locale")?;
        let verified = row.try_get::<Option<bool>, &str>("verified")?;
        let email = row.try_get::<Option<String>, &str>("email")?;
        let flags_string = row.try_get::<Option<String>, &str>("flags")?;
        let flags = flags_string.map(|string| UserFlags::from_bits(string.parse::<u64>().unwrap()).unwrap());
        let premium_type_string = row.try_get::<Option<String>, &str>("premium_type")?;
        let premium_type = premium_type_string.map(|string| match &*string {
            "None" => PremiumType::None,
            "Nitro" => PremiumType::Nitro,
            "Nitro Classic" => PremiumType::NitroClassic,
            _ => unreachable!(),
        });
        let public_flags_string = row.try_get::<Option<String>, &str>("public_flags")?;
        let public_flags = public_flags_string.map(|string| UserFlags::from_bits(string.parse::<u64>().unwrap()).unwrap());

        Ok(Self {
            accent_colour,
            avatar,
            banner,
            bot,
            discriminator,
            email,
            flags,
            id,
            locale,
            mfa_enabled,
            username,
            premium_type,
            public_flags,
            system,
            verified
        })
    }
}
