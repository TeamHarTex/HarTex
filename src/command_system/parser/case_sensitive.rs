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

use unicase::UniCase;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
crate enum CaseSensitive {
    True(UniCase<String>),
    False(String)
}

impl AsRef<str> for CaseSensitive {
    fn as_ref(&self) -> &str {
        match self {
            Self::True(unicase_string) => unicase_string.as_str(),
            Self::False(string) => string.as_str()
        }
    }
}

impl PartialEq<str> for CaseSensitive {
    fn eq(&self, other: &str) -> bool {
        match self {
            Self::True(unicase_string) => unicase_string == &UniCase::new(other),
            Self::False(string) => string == other,
        }
    }
}
