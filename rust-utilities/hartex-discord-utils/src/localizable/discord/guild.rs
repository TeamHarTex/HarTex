/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use unic_langid::LanguageIdentifier;
use hartex_discord_core::discord::model::guild::DefaultMessageNotificationLevel;
use hartex_localization_core::create_bundle;
use hartex_localization_core::handle_errors;
use hartex_localization_macros::bundle_get;

use crate::localizable::Localizable;

impl Localizable for DefaultMessageNotificationLevel {
    fn localize(&self, locale: Option<LanguageIdentifier>) -> miette::Result<String> {
        let bundle = create_bundle(
            locale,
            &["discord-frontend"],
        )?;

        Ok(match self {
            Self::All => {
                bundle_get!(bundle."default-message-notification-level-all": message, out [default_message_notification_level_all, errors]);
                handle_errors(errors)?;

                default_message_notification_level_all.to_owned()
            }
            Self::Mentions => {
                bundle_get!(bundle."default-message-notification-level-mentions": message, out [default_message_notification_level_mentions, errors]);
                handle_errors(errors)?;

                default_message_notification_level_mentions.to_owned()
            }
            _ => {
                bundle_get!(bundle."default-message-notification-level-unknown": message, out [default_message_notification_level_unknown, errors]);
                handle_errors(errors)?;

                default_message_notification_level_unknown.to_owned()
            }
        })
    }
}
