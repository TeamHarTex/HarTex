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
    borrow::Cow,
    slice::{
        Iter,
        IterMut
    }
};

use crate::{
    command_system::{
        parser::{
            case_sensitive::CaseSensitive
        },
        cfg::CaseSensitivity
    },
};

#[derive(Clone, Debug, Default)]
crate struct CommandParserConfiguration<'a> {
    crate commands: Vec<CaseSensitive>,
    crate command_prefixes: Vec<Cow<'a, str>>
}

impl<'a> CommandParserConfiguration<'a> {
    #[allow(dead_code)]
    crate fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    crate fn commands(&self) -> Commands<'_> {
        Commands {
            iter: self.commands.iter()
        }
    }

    #[allow(dead_code)]
    crate fn commands_mut(&'a mut self) -> CommandsMut<'a> {
        CommandsMut {
            iter: self.commands.iter_mut()
        }
    }

    #[allow(dead_code)]
    crate fn command_prefixes(&self) -> Prefixes<'_> {
        Prefixes {
            iter: self.command_prefixes.iter(),
        }
    }

    #[allow(dead_code)]
    crate fn command_prefixes_mut(&'a mut self) -> PrefixesMut<'_> {
        PrefixesMut {
            iter: self.command_prefixes.iter_mut(),
        }
    }

    crate fn add_command(&mut self, name: impl Into<String>, case_sensitive: CaseSensitivity) -> bool {
        self._internal_add_command(name.into(), case_sensitive)
    }

    fn _internal_add_command(&mut self, name: String, case_sensitive: CaseSensitivity) -> bool {
        let command = match case_sensitive {
            CaseSensitivity::False => CaseSensitive::False(name),
            CaseSensitivity::True => CaseSensitive::True(name.into())
        };

        if self.commands.contains(&command) {
            false
        } else {
            self.commands.push(command);
            true
        }
    }

    crate fn add_prefix(&mut self, prefix: impl Into<Cow<'a, str>>) -> bool {
        let prefix = prefix.into();

        if self.command_prefixes.contains(&prefix) {
            false
        } else {
            self.command_prefixes.push(prefix);

            true
        }
    }

    #[allow(dead_code)]
    crate fn remove_prefix(&mut self, prefix: impl Into<Cow<'a, str>>) -> Option<Cow<'a, str>> {
        let needle = prefix.into();
        let pos = self.command_prefixes.iter().position(|e| *e == needle)?;

        Some(self.command_prefixes.remove(pos))
    }
}

crate struct Commands<'a> {
    iter: Iter<'a, CaseSensitive>,
}

impl<'a> Iterator for Commands<'a> {
    type Item = &'a CaseSensitive;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for Commands<'a> {}

crate struct CommandsMut<'a> {
    iter: IterMut<'a, CaseSensitive>,
}

impl<'a> Iterator for CommandsMut<'a> {
    type Item = &'a mut CaseSensitive;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for CommandsMut<'a> {}

crate struct Prefixes<'a> {
    iter: Iter<'a, Cow<'a, str>>,
}

impl<'a> Iterator for Prefixes<'a> {
    type Item = &'a Cow<'a, str>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for Prefixes<'a> {}

crate struct PrefixesMut<'a> {
    iter: IterMut<'a, Cow<'a, str>>,
}

impl<'a> Iterator for PrefixesMut<'a> {
    type Item = &'a mut Cow<'a, str>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for PrefixesMut<'a> {}
