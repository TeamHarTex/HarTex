use hartex_discord_entitycache_macros::entity;
use hartex_discord_utils::DATABASE_POOL;
use tokio_postgres::GenericClient;
pub struct GuildOverride {
    pub default_message_notifications: twilight_model::guild::DefaultMessageNotificationLevel,
    pub explicit_content_filter: twilight_model::guild::ExplicitContentFilter,
    pub features: Vec<twilight_model::guild::GuildFeature>,
    pub icon: Option<twilight_model::util::image_hash::ImageHash>,
    pub large: bool,
    pub name: String,
    pub owner_id: twilight_model::id::Id<twilight_model::id::marker::UserMarker>,
    pub id: twilight_model::id::Id<twilight_model::id::marker::GuildMarker>,
}
impl GuildOverride {}
#[automatically_derived]
impl hartex_discord_entitycache_core::traits::Entity for GuildOverride {
    type Id = twilight_model::id::Id<twilight_model::id::marker::GuildMarker>;
    fn id(&self) -> <Self as hartex_discord_entitycache_core::traits::Entity>::Id {
        self.id
    }
}
impl From<twilight_model::guild::Guild> for GuildOverride {
    fn from(model: twilight_model::guild::Guild) -> Self {
        Self {
            default_message_notifications: model.default_message_notifications,
            explicit_content_filter: model.explicit_content_filter,
            features: model.features,
            icon: model.icon,
            large: model.large,
            name: model.name,
            owner_id: model.owner_id,
            id: model.id,
        }
    }
}
fn main() {}
