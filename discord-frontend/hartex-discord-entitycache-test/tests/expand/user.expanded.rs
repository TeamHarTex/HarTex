use hartex_discord_entitycache_macros::entity;
use tokio_postgres::GenericClient;
pub struct UserExpand {
    pub accent_color: Option<u32>,
    pub avatar: Option<twilight_model::util::image_hash::ImageHash>,
    pub bot: bool,
    pub discriminator: u16,
    pub name: String,
    pub id: twilight_model::id::Id<twilight_model::id::marker::UserMarker>,
}
impl UserExpand {}
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
