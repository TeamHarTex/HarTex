use hartex_discord_commands_macros::CommandMetadata;
pub struct Derive;
impl hartex_discord_commands_core::CommandMetadata for Derive {
    fn command_type(&self) -> u8 {
        1
    }
    fn description(&self) -> String {
        String::from("derive macro expand test")
    }
    fn interaction_only(&self) -> bool {
        true
    }
    fn name(&self) -> String {
        String::from("derive")
    }
}
