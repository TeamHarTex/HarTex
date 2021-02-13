///  Copyright 2020 - 2021 The HarTex Project Developers
///
///  Licensed under the Apache License, Version 2.0 (the "License");
///  you may not use this file except in compliance with the License.
///  You may obtain a copy of the License at
///
///      http://www.apache.org/licenses/LICENSE-2.0
///
///  Unless required by applicable law or agreed to in writing, software
///  distributed under the License is distributed on an "AS IS" BASIS,
///  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
///  See the License for the specific language governing permissions and
///  limitations under the License.

crate mod cfg;
mod command;
mod command_context;
mod error;
crate mod events;
mod framework;
crate mod parser;
crate mod precommand_checks;
mod precommand_check_parameters;

crate use command::Command;
crate use command_context::{
    CommandContext,
    CommandContextRef
};
crate use error::CommandError;
crate use framework::CommandFramework;
crate use precommand_check_parameters::{
    PrecommandCheckParameters,
    PrecommandCheckParametersBuilder
};
