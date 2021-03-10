//!  Copyright 2020 - 2021 The HarTex Project Developers
//!
//!  Licensed under the Apache License, Version 2.0 (the "License");
//!  you may not use this file except in compliance with the License.
//!  You may obtain a copy of the License at
//!
//!      http://www.apache.org/licenses/LICENSE-2.0
//!
//!  Unless required by applicable law or agreed to in writing, software
//!  distributed under the License is distributed on an "AS IS" BASIS,
//!  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//!  See the License for the specific language governing permissions and
//!  limitations under the License.

crate macro execute_command {
    ($command_struct:ty, $context:expr, $arguments:expr, $cache:expr, $http_client:expr, $message: expr, $emitter:expr) => {
        match $command_struct::execute_command($context, $arguments, $cache).await {
            Ok(()) => {
                let guild = match $http_client.guild($message.guild_id.unwrap()).await? {
                    Some(guild) => guild.name,
                    None => String::new()
                };

                $emitter.event($crate::command_system::events::events::SystemEvent::CommandExecuted(box CommandExecuted {
                    command: $command_struct::fully_qualified_name(),
                    guild_name: guild,
                    context: $context
                }))
            },
            Err(error) => {
                $emitter.event($crate::command_system::events::eventsSystemEvent::CommandFailed(box CommandFailed {
                    command: $command_struct::fully_qualified_name(),
                    error: format!("{}", error)
                }))
            }
        }
    }

    ($command_struct:ty, $precommand_checks:expr, $precommand_checks_parameters:expr, $context:expr,
        $arguments:expr, $cache:expr, $http_client:expr, $message: expr, $emitter:expr) => {
        match $command_struct::precommand_checks($context, $precommand_checks_parameters, $precommand_checks).await {
            Ok(()) => {
                match $command_struct::execute_command($context, $arguments, $cache).await {
                    Ok(()) => {
                        let guild = match $http_client.guild($message.guild_id.unwrap()).await? {
                            Some(guild) => guild.name,
                            None => String::new()
                        };

                        $emitter.event($crate::command_system::events::events::SystemEvent::CommandExecuted(box CommandExecuted {
                            command: $command_struct::fully_qualified_name(),
                            guild_name: guild,
                            context: $context
                        }))
                    },
                    Err(error) => {
                        $emitter.event($crate::command_system::events::eventsSystemEvent::CommandFailed(box CommandFailed {
                            command: $command_struct::fully_qualified_name(),
                            error: format!("{}", error)
                        }))
                    }
                }
            }
        }
    }
}
