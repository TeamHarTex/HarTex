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

mod blocked_mentions_detection;
mod blocked_nickname_detection;
mod blocked_words_or_tokens_detection;
mod domain_detection;
mod invite_detection;
mod zalgo_detection;
mod zalgo_nickname_detection;

crate use blocked_mentions_detection::BlockedMentionsDetectionTask;
crate use blocked_nickname_detection::BlockedNicknameDetectionTask;
crate use blocked_words_or_tokens_detection::BlockedWordsOrTokensDetectionTask;
crate use domain_detection::DomainDetectionTask;
crate use invite_detection::InviteDetectionTask;
crate use zalgo_detection::ZalgoDetectionTask;
crate use zalgo_nickname_detection::ZalgoNicknameDetectionTask;
