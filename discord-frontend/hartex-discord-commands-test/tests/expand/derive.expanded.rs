use hartex_discord_commands_core::CommandMetadata;
#[metadata(command_type = 1)]
#[metadata(description = "derive macro expand test")]
#[metadata(interaction_only = true)]
#[metadata(name = "derive")]
pub struct Derive;
const _: () = {
    extern crate hartex_discord_commands_core as _commands_core;
    #[automatically_derived]
    impl _commands_core::CommandMetadata for Derive {
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
};
