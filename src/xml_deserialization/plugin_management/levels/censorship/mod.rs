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

extern crate serde;
extern crate quick_xml;

use crate::xml_deserialization::plugin_management::{
    levels::Level,
    models::censorship::{
        blacklisted_domains::BlacklistedDomains,
        blacklisted_invite_codes::BlacklistedInviteCodes,
        domains_channel_whitelist::DomainsChannelWhitelist,
        invites_channel_whitelist::InvitesChannelWhitelist,
        prohibited_words::ProhibitedWords,
        whitelisted_domains::WhitelistedDomains,
        whitelisted_guild_invites::WhitelistedGuildInvites,
        whitelisted_invite_codes::WhitelistedInviteCodes,
        zalgo_channel_whitelist::ZalgoChannelWhitelist,
        CensoredUri
    }
};

#[derive(Debug, Clone, Serialize, Deserialize)]
crate struct CensorshipLevel {
    #[serde(rename = "Level")]
    crate level: Level,

    #[serde(rename = "FilterZalgo", default)]
    crate filter_zalgo: Option<bool>,

    #[serde(rename = "ZalgoFilterWhitelistedChannels", default)]
    crate zalgo_channel_whitelist: Option<ZalgoChannelWhitelist>,

    #[serde(rename = "FilterInviteLinks", default)]
    crate filter_invite_links: Option<bool>,

    #[serde(rename = "WhitelistedGuildInvites", default)]
    crate whitelisted_guild_invites: Option<WhitelistedGuildInvites>,

    #[serde(rename = "InvitesChannelWhitelist", default)]
    crate invites_channel_whitelist: Option<InvitesChannelWhitelist>,

    #[serde(rename = "WhitelistedInviteCodes", default)]
    crate whitelisted_invite_codes: Option<WhitelistedInviteCodes>,

    #[serde(rename = "BlacklistedInviteCodes", default)]
    crate blacklisted_invite_codes: Option<BlacklistedInviteCodes>,

    #[serde(rename = "FilterDomains", default)]
    crate filter_domains: Option<bool>,

    #[serde(rename = "DomainsChannelWhitelist", default)]
    crate domains_channel_whitelist: Option<DomainsChannelWhitelist>,

    #[serde(rename = "WhitelistedDomains", default)]
    crate whitelisted_domains: Option<WhitelistedDomains>,

    #[serde(rename = "BlacklistedDomains")]
    crate blacklisted_domains: Option<BlacklistedDomains>,

    #[serde(rename = "ProhibitedWords", default)]
    crate prohibited_words: Option<ProhibitedWords>
}
