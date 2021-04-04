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

use std::{
    lazy::{
        SyncLazy
    },
    panic::{
        PanicInfo,
        set_hook,
        take_hook
    }
};

use crate::{
    command_system::CommandContext,
    logging::logger::Logger,
    system::{
        terminal::Ansi256,
        SystemResult
    },
    utilities::{
        constants::{
            bot_support_server
        }
    }
};

crate static RUST_DEFAULT_PANIC_HOOK: SyncLazy<Box<dyn Fn(&PanicInfo<'_>) + Send + Sync + 'static>> =
    SyncLazy::new(|| {
        let hook = take_hook();

        set_hook(box |info| {
            hartex_begin_panic(info)
        });

        hook
    });

crate fn hartex_begin_panic(panic_info: &PanicInfo<'_>) {
    Logger::log_error("An unexpected panic has taken place. See below for more information:",
    "system::panicking::hartex_begin_panic");

    eprintln!("{}error: internal bot error: unexpected panic{}\n", Ansi256 { colour: 1 }, Ansi256::reset());
    eprintln!("note: the bot unexpectedly panicked. this is a bug.\n");
    eprintln!("note: we would appreciate a bug report: https://github.com/HT-Studios/HarTex-rust-discord-bot/issues/new?labels=B-IBE&template=internal-bot-error.md\n");

    (*RUST_DEFAULT_PANIC_HOOK)(panic_info);
}
