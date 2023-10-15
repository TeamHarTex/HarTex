use hartex_discord_entitycache_macros::entity;
pub struct UserExpand {
    accent_color: Option<u32>,
    avatar: Option<twilight_model::util::image_hash::ImageHash>,
    bot: bool,
    discriminator: u16,
    name: String,
    id: twilight_model::id::Id<twilight_model::id::marker::UserMarker>,
}
fn main() {}
