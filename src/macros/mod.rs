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
    ($command_struct:ident, $context:expr, $arguments:expr, $cache:expr, $http_client:expr,
        $message: expr, $emitter:expr, $command_name:literal) => {
        use $crate::command_system::Command;

        match <$command_struct>::execute_command($context, $arguments, $cache).await {
            Ok(()) => {
                let guild_name = $http_client.guild($message.guild_id.unwrap()).await?.map_or("unknown".to_string(), |guild| guild.name);

                $emitter.event($crate::command_system::events::events::SystemEvent::CommandExecuted(box $crate::system::model::payload::CommandExecuted {
                    command: $command_name,
                    guild_name,
                    context: $context
                }))
            },
            Err(error) => {
                $emitter.event($crate::command_system::events::events::SystemEvent::CommandFailed(box $crate::system::model::payload::CommandFailed {
                    command: $command_name,
                    error: format!("{}", error)
                }));

                return Err(error);
            }
        }
    },

    ($command_struct:ident, $precommand_checks:expr, $precommand_checks_parameters:expr, $context:expr,
        $arguments:expr, $cache:expr, $http_client:expr, $message: expr, $emitter:expr, $command_name:literal) => {
        use $crate::command_system::Command;

        match <$command_struct>::precommand_checks($context, $precommand_checks_parameters, $precommand_checks).await {
            Ok(()) => {
                match <$command_struct>::execute_command($context, $arguments, $cache).await {
                    Ok(()) => {
                        let guild_name = $http_client.guild($message.guild_id.unwrap()).await?.map_or("unknown".to_string(), |guild| guild.name);

                        $emitter.event($crate::command_system::events::events::SystemEvent::CommandExecuted(box $crate::system::model::payload::CommandExecuted {
                            command: $command_name,
                            guild_name,
                            context: $context
                        }))
                    },
                    Err(error) => {
                        $emitter.event($crate::command_system::events::events::SystemEvent::CommandFailed(box $crate::system::model::payload::CommandFailed {
                            command: $command_name,
                            error: format!("{}", error)
                        }));

                        return Err(error);
                    }
                }
            },
            Err(error) => {
                $emitter.event($crate::command_system::events::events::SystemEvent::CommandFailed(box $crate::system::model::payload::CommandFailed {
                    command: $command_name,
                    error: format!("{}", error)
                }));

                return Err(error);
            }
        }
    }
}
