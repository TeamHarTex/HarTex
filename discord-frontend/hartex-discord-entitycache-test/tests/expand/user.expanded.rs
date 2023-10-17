use hartex_discord_entitycache_macros::entity;
pub struct UserExpand {
    accent_color: Option<u32>,
    avatar: Option<twilight_model::util::image_hash::ImageHash>,
    bot: bool,
    discriminator: u16,
    name: String,
    id: twilight_model::id::Id<twilight_model::id::marker::UserMarker>,
}
#[automatically_derived]
impl hartex_discord_entitycache_core::traits::Entity for UserExpand {
    type Id = twilight_model::id::Id<twilight_model::id::marker::UserMarker>;
    fn id(&self) -> <Self as hartex_discord_entitycache_core::traits::Entity>::Id {
        self.id
    }
}
impl From<twilight_model::user::User> for UserExpand {
    fn from(model: twilight_model::user::User) -> Self {
        Self {
            accent_color: model.accent_color,
            avatar: model.avatar,
            bot: model.bot,
            discriminator: model.discriminator,
            name: model.name,
            id: model.id,
        }
    }
}
fn main() {}
