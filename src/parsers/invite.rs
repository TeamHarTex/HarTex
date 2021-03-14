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

use super::{
    error::ParseError,
    Parser,
    ParseResult
};

use regex::Regex;

crate struct InviteParser;

#[derive(Clone, Debug)]
crate struct Invite {
    crate code: String
}

impl InviteParser {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for InviteParser {
    fn default() -> Self {
        Self
    }
}

impl Parser for InviteParser {
    type Output = Invite;

    fn parse(&self, input: String) -> ParseResult<Self::Output> {
        let regex = Regex::new(r#"((https://)?discord\.gg/)(([a-z]|[A-Z]|[0-9])+)"#).unwrap();

        if let Some(captures) = regex.captures(&input) {
            dbg!(captures.get(0));
            dbg!(captures.get(1));
            dbg!(captures.get(2));
            dbg!(captures.get(3));
            dbg!(captures.get(4));
        }

        Err(ParseError("Intentional error".to_string()))
    }
}
