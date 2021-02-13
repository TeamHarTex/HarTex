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

use std::{
    fmt::{
        Display,
        Formatter,
        Result
    }
};

crate struct Ansi256 {
    crate colour: u16
}

impl Ansi256 {
    crate fn reset() -> String {
        "\x1B[0m".to_string()
    }
}

impl Display for Ansi256 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        return write!(f, "\x1B[38;5;{}m", self.colour);
    }
}
