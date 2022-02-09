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

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config as LoggerConfig, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;

pub fn init() {
    let conf = LoggerConfig::builder()
        .appender(
            Appender::builder().build(
                "stdout",
                Box::new(
                    ConsoleAppender::builder()
                        .encoder(Box::new(PatternEncoder::new(
                            "{h({d(%Y-%m-%d %H:%M:%S %Z)(local):>30} {l:>6} {M}  {m})}{n}",
                        )))
                        .build(),
                ),
            ),
        )
        .logger(Logger::builder().build("gateway", LevelFilter::Trace))
        .logger(Logger::builder().build("hyper", LevelFilter::Off))
        .logger(Logger::builder().build("mio", LevelFilter::Off))
        .logger(Logger::builder().build("rustls", LevelFilter::Off))
        .logger(Logger::builder().build("tokio_tungstenite", LevelFilter::Off))
        .logger(Logger::builder().build("tokio_util", LevelFilter::Off))
        .logger(Logger::builder().build("tungstenite", LevelFilter::Off))
        .logger(Logger::builder().build("want", LevelFilter::Off))
        .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
        .expect("could not build log4rs config");

    log4rs::init_config(conf).expect("could not initialize log4rs");
}
