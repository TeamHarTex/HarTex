/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use base::discord::model::application::command::{Command, CommandType};
use base::discord::model::application::interaction::Interaction;
use base::discord::model::id::Id;

use crate::{BaseInteraction, HandleInteractionFuture};

pub struct PingCommand;

impl BaseInteraction for PingCommand {
    fn commands() -> Vec<Command> {
        #[cfg(stable)]
        let application_id = Some(Id::new(936431574310879332));
        #[cfg(not(stable))]
        let application_id = Some(Id::new(936432439767740436));

        vec![
            Command {
                application_id,
                default_member_permissions: None,
                dm_permission: Some(false),
                description: String::new(),
                description_localizations: None,
                guild_id: None,
                id: None,
                kind: CommandType::ChatInput,
                name: String::from("ping"),
                name_localizations: None,
                options: vec![],
                version: Id::new(1),
            }
        ]
    }

    fn handle(interation: Interaction) -> HandleInteractionFuture {
        todo!()
    }
}
