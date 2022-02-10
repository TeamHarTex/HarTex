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

use std::collections::HashMap;
use std::env;
use std::fs;
use std::ops::Index;

use base::error::{Error, ErrorKind, Result};

pub enum EnvVarKind {
    Common,
}

#[derive(Clone, Debug)]
pub struct EnvVars<'a> {
    vars: HashMap<&'a str, EnvVarValue>,
}

impl<'a> EnvVars<'a> {
    pub fn get(kind: EnvVarKind) -> Result<Self> {
        Self::__load(kind)
    }

    pub(in crate) fn __load(kind: EnvVarKind) -> Result<Self> {
        match kind {
            EnvVarKind::Common => {
                let file = fs::read_to_string("CommonEnv.vars")?;
                let file = file.trim_end().to_string();

                let lines = file
                    .lines()
                    .filter(|line| !line.starts_with(";") && !line.is_empty());

                for line in lines {
                    let split = line.split(" ").collect::<Vec<_>>();
                    env::set_var(split[1], split[2]);
                }

                Self::__common_vars()
            }
        }
    }

    pub(in crate) fn __common_vars() -> Result<Self> {
        let mut vars = HashMap::new();

        let token = env::var("BOT_TOKEN")?;
        vars.insert("BOT_TOKEN", EnvVarValue::String(token));

        let event_server_auth = env::var("EVENT_SERVER_AUTH")?;
        vars.insert("EVENT_SERVER_AUTH", EnvVarValue::String(event_server_auth));

        let event_server_port_string = env::var("EVENT_SERVER_PORT")?;
        let event_server_port = if let Ok(port) = event_server_port_string.parse::<u16>() {
            port
        } else {
            return Err(Error::from(ErrorKind::PortNotNumber {
                name: String::from("EVENT_SERVER_PORT"),
            }));
        };
        vars.insert("EVENT_SERVER_PORT", EnvVarValue::U16(event_server_port));

        Ok(Self { vars })
    }
}

impl<'a> Index<&str> for EnvVars<'a> {
    type Output = EnvVarValue;

    fn index(&self, index: &str) -> &Self::Output {
        self.vars.get(index).unwrap()
    }
}

#[derive(Clone, Debug)]
pub enum EnvVarValue {
    String(String),
    U16(u16),
}
