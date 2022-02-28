//! Users in the Discord entity cache.

use base::discord::model::id::{marker::UserMarker, Id};
use base::discord::model::user::{PremiumType, UserFlags};
use base::discord::model::util::ImageHash;
use cache_base::Entity;

/// This is basically identical to a regular cached user.
///
/// This structure is here only for a separate cache for the current user (aka the bot itself).
pub struct CachedCurrentUser {
    accent_colour: Option<u64>,
    avatar: Option<ImageHash>,
    banner: Option<ImageHash>,
    bot: bool,
    discriminator: String,
    email: Option<String>,
    flags: Option<UserFlags>,
    id: Id<UserMarker>,
    locale: Option<String>,
    mfa_enabled: Option<bool>,
    username: String,
    premium_type: Option<PremiumType>,
    public_flags: Option<UserFlags>,
    system: Option<bool>,
    verified: Option<bool>,
}

impl CachedCurrentUser {
    pub fn accent_colour(&self) -> Option<u64> {
        self.accent_colour
    }

    pub fn avatar(&self) -> Option<ImageHash> {
        self.avatar
    }

    pub fn banner(&self) -> Option<ImageHash> {
        self.banner
    }

    pub fn bot(&self) -> bool {
        self.bot
    }

    pub fn discriminator(&self) -> &str {
        &self.discriminator
    }

    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    pub fn flags(&self) -> Option<UserFlags> {
        self.flags
    }

    pub fn locale(&self) -> Option<&str> {
        self.locale.as_deref()
    }

    pub fn mfa_enabled(&self) -> Option<bool> {
        self.mfa_enabled
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn premium_type(&self) -> Option<PremiumType> {
        self.premium_type
    }

    pub fn public_flags(&self) -> Option<UserFlags> {
        self.public_flags
    }

    pub fn system(&self) -> Option<bool> {
        self.system
    }

    pub fn verified(&self) -> Option<bool> {
        self.verified
    }
}

impl Entity for CachedCurrentUser {
    type Id = Id<UserMarker>;

    fn id(&self) -> Self::Id {
        self.id
    }
}
