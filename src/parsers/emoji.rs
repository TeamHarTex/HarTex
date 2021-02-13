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

use super::{
    error::ParseError,
    Parser,
    ParseResult
};

use regex::Regex;

crate struct EmojiParser;

#[derive(Clone, Debug)]
crate struct Emoji {
    crate name: String,
    crate id: u64,
    crate animated: bool
}

impl EmojiParser {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for EmojiParser {
    fn default() -> Self {
        Self
    }
}

impl Parser for EmojiParser {
    type Output = Emoji;

    fn parse(&self, input: String) -> ParseResult<Self::Output> {
        let regex = Regex::new(r#"<(a?):(\w+):(\d+)>"#).unwrap();

        if let Some(captures) = regex.captures(&input) {
            let mut is_animated = false;
            let mut name = String::new();
            let mut emoji_id = 0u64;

            if let Some(c) = captures.get(1) {
                match c.as_str() {
                    "a" => {
                        is_animated = true;

                        if let Some(n) = captures.get(2) {
                            name = String::from(n.as_str());
                        }
                        else {
                            return Err(ParseError("Could not parse emoji name.".to_string()));
                        }

                        if let Some(id) = captures.get(3) {
                            if let Ok(id_num) = id.as_str().parse() {
                                emoji_id = id_num;
                            }
                            else if let Err(error) = id.as_str().parse::<u64>() {
                                return Err(ParseError(format!("Could not parse emoji id: {}", error)));
                            }
                            else {
                                unreachable!()
                            }
                        }
                    },
                    _ => {
                        is_animated = false;

                        if let Some(n) = captures.get(2) {
                            name = String::from(n.as_str());
                        }
                        else {
                            return Err(ParseError("Could not parse emoji name.".to_string()));
                        }

                        if let Some(id) = captures.get(3) {
                            if let Ok(id_num) = id.as_str().parse() {
                                emoji_id = id_num;
                            }
                            else if let Err(error) = id.as_str().parse::<u64>() {
                                return Err(ParseError(format!("Could not parse emoji id: {}", error)));
                            }
                            else {
                                unreachable!()
                            }
                        }
                    }
                }
            }

            Ok(Emoji {
                name,
                id: emoji_id,
                animated: is_animated
            })
        }
        else {
            Err(ParseError("Could not parse provided string.".to_string()))
        }
    }
}
