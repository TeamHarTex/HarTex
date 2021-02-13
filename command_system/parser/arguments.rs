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
        Debug,
        Formatter,
        Result as FmtResult
    },
    str::CharIndices,
};

#[derive(Clone)]
crate struct Arguments<'a> {
    buf: &'a str,
    indices: CharIndices<'a>,
    idx: usize,
}

impl<'a> Arguments<'a> {
    crate fn new(buf: &'a str) -> Self {
        Self::from(buf)
    }

    #[allow(dead_code)]
    crate fn as_str(&self) -> &str {
        self.buf
    }

    crate fn into_remainder(self) -> Option<&'a str> {
        self.buf.get(self.idx..)
    }
}

impl<'a> From<&'a str> for Arguments<'a> {
    fn from(buf: &'a str) -> Self {
        Self {
            buf: buf.trim(),
            indices: buf.trim().char_indices(),
            idx: 0,
        }
    }
}

impl<'a> Debug for Arguments<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Arguments")
            .field("buf", &self.buf)
            .field("idx", &self.idx)
            .finish()
    }
}

impl<'a> Iterator for Arguments<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx > self.buf.len() {
            return None;
        }

        let mut start_idx = self.idx;
        let mut quoted = false;
        let mut started = false;

        while let Some((i, ch)) = self.indices.next() {
            if quoted {
                if ch == '"' {
                    let v = self.buf.get(start_idx..i);
                    self.idx = i + 1;

                    return v.map(str::trim);
                }
            } else if ch == ' ' {
                if started {
                    let v = self.buf.get(start_idx..i);
                    self.idx = i + 1;

                    return v.map(str::trim);
                } else {
                    self.idx = i;
                    start_idx = i;
                    started = true;
                    continue;
                }
            } else if ch == '"' {
                start_idx = i + 1;
                quoted = true;
            }

            self.idx = i;
            started = true;
        }

        self.idx = usize::max_value();

        match self.buf.get(start_idx..) {
            Some("") | None => None,
            Some(v) => Some(v.trim()),
        }
    }
}