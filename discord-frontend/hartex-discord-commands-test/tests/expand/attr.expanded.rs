use hartex_discord_commands_core::metadata;
extern crate hartex_discord_commands_core as _commands_core;
pub struct Attr;
#[automatically_derived]
impl _commands_core::traits::CommandMetadata for Attr {
    fn command_type(&self) -> u8 {
        1
    }
    fn interaction_only(&self) -> bool {
        true
    }
    fn name(&self) -> String {
        String::from("attr")
    }
}
