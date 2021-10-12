//! # The `cdn` Module
//!
//! This module implements a wrapper of the Discord CDN.

use std::fmt::{
    self,
    Display,
    Formatter
};

use hartex_core::discord::model::id::{
    GuildId,
    UserId
};

/// # Struct `Cdn`
///
/// The "namespace" for various CDN endpoints
pub struct Cdn;

impl Cdn {
    pub fn default_user_avatar(discriminator: u16) -> String {
        format!(
            "https://cdn.discordapp.com/embed/avatars/{discriminator}.png",
            discriminator = discriminator % 5
        )
    }

    pub fn guild_icon(guild_id: GuildId, icon_hash: String, format: CdnResourceFormat) -> String {
        format!("https://cdn.discordapp.com/icons/{guild_id}/{icon_hash}{format}")
    }

    pub fn user_avatar(user_id: UserId, avatar_hash: String, format: CdnResourceFormat) -> String {
        format!("https://cdn.discordapp.com/avatars/{user_id}/{avatar_hash}{format}")
    }
}

/// # Enumeration `CdnResourceFormat`
///
/// The format of a CDN resource
pub enum CdnResourceFormat {
    /// # Enumeration Variant `CdnResourceFormat::GIF`
    ///
    /// A GIF - `.gif`
    GIF,

    /// # Enumeration Variant `CdnResourceFormat::JPEG`
    ///
    /// A JPEG - `.jpeg`
    JPEG,

    /// # Enumeration Variant `CdnResourceFormat::PNG`
    ///
    /// A PNG - `.png`
    PNG,

    /// # Enumeration Variant `CdnResourceFormat::WebP`
    ///
    /// A WebP - `.webp`
    WebP
}

impl Display for CdnResourceFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::GIF => f.write_str(".gif"),
            Self::JPEG => f.write_str(".jpeg"),
            Self::PNG => f.write_str(".png"),
            Self::WebP => f.write_str(".webp")
        }
    }
}
