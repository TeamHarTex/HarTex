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

mod role_add;
mod role_global_add;
mod role_global_remove;
mod role_remove;
mod roleinfo;

crate mod noroles_manipulation;

crate use role_add::RoleAddCommand;
crate use role_global_add::RoleGlobalAddCommand;
crate use role_global_remove::RoleGlobalRemoveCommand;
crate use role_remove::RoleRemoveCommand;
crate use roleinfo::RoleinfoCommand;
