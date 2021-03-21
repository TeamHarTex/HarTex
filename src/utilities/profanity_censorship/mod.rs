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

// Original Code from: censor 0.2.0, with appropriate amendments.

use std::{
    collections::{
        hash_set,
        BTreeSet,
        HashMap,
        HashSet
    },
    lazy::{
        SyncLazy
    },
    ops::{
        Add,
        AddAssign,
        Sub,
        SubAssign
    }
};

crate mod character_aliases;
crate mod word_sets;

crate enum ProfanityCensorshipOptions {
    UseStandardWordSetOnly,

    UseSexWordSetOnly,

    UseZealousWordSetOnly,

    UseCustomWordSet(HashSet<String>)
}

impl ProfanityCensorshipOptions {
    crate fn empty() -> Self {
        Self::UseCustomWordSet(HashSet::new())
    }

    crate fn custom<I, T>(words: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String> {
        Self::UseCustomWordSet(words.into_iter().map(Into::into).collect())
    }

    crate fn check(&self, text: &str) -> bool {
        !self.bad_characters(text, 0, 0).is_empty()
    }

    crate fn bad_characters(&self, text: &str, start_offset: usize, end_offset: usize) -> HashSet<usize> {
        let lowercase = text.to_lowercase();
        let sizes = self.iter().map(|s| s.len()).collect::<BTreeSet<usize>>();
        let (alphabetaic_only, alphabetic_hashmap) = remove_non_alphabetic(&lowercase);
        let bad_alphabetic_characters = self.__internal_bad_characters(
            &alphabetaic_only,
            &alphabetic_hashmap,
            &sizes,
            start_offset,
            end_offset
        );

        let (aliase_without_whitespace, alias_without_whitespaces_hashmap) = remove_whitespace(&alias(&lowercase));
        let bad_alias_without_whitespace_characters = self.__internal_bad_characters(
            &aliase_without_whitespace,
            &alias_without_whitespaces_hashmap,
            &sizes,
            start_offset,
            end_offset
        );

        let (alias_alphabetic, alias_alphabetic_hashmap) = remove_non_alphabetic(&alias(&lowercase));
        let bad_alias_alphabetic_characters = self.__internal_bad_characters(
            &alias_alphabetic,
            &alias_alphabetic_hashmap,
            &sizes,
            start_offset,
            end_offset
        );

        bad_alphabetic_characters
            .into_iter()
            .chain(bad_alias_without_whitespace_characters)
            .chain(bad_alias_alphabetic_characters)
            .collect()
    }

    fn __internal_bad_characters(&self,
                                 text: &str,
                                 hashmap: &HashMap<usize, usize>,
                                 sizes: &BTreeSet<usize>,
                                 start_offset: usize,
                                 end_offset: usize) -> HashSet<usize> {
        let (deduplicated, dedup_hashmap) = deduplicate_string(text);
        let mut set = HashSet::new();

        for &size in sizes.iter().rev() {
           for word in self.iter().filter(|string| string.len() == size) {
               for (index, _) in text.match_indices(word.as_str()) {
                   for jndex in start_offset..word.len().saturating_sub(end_offset) {
                       let kndex = index + jndex;

                       if let Some(k) = hashmap.get(&kndex) {
                           set.insert(*k);
                       }
                   }
               }

               for (index, _) in deduplicated.match_indices(word.as_str()) {
                   for jndex in start_offset..word.len().saturating_sub(end_offset) {
                       let kndex = index + jndex;

                       if let Some(vector) = dedup_hashmap.get(&k) {
                           for l in vector {
                               if let Some(k) = hashmap.get(l) {
                                   set.insert(*k);
                               }
                           }
                       }
                   }
               }
           }
        }

        set
    }

    pub fn set(&self) -> &HashSet<String> {
        match self {
            Self::UseStandardWordSetOnly => &*word_sets::STANDARD_WORD_SET,
            Self::UseZealousWordSetOnly => &*word_sets::ZEALOUS_WORD_SET,
            Self::UseSexWordSetOnly => &*word_sets::SEX_WORD_SET,
            Self::UseCustomWordSet(custom) => custom
        }
    }

    pub fn iter(&self) -> hash_set::Iter<String> {
        self.set().iter()
    }
}

impl Default for ProfanityCensorshipOptions {
    fn default() -> Self {
        Self::UseStandardWordSetOnly
    }
}

impl PartialEq for ProfanityCensorshipOptions {
    fn eq(&self, other: Self) -> bool {
        self.set() == other.set()
    }
}

impl AddAssign for ProfanityCensorshipOptions {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::UseCustomWordSet(self.set().union(rhs.set()).cloned().collect())
    }
}

impl<S> AddAssign<S> for ProfanityCensorshipOptions
where
    S: Into<String> {
    fn add_assign(&mut self, rhs: S) {
        *self = Self::UseCustomWordSet(self.iter().cloned().chain(Some(rhs.into())).collect())
    }
}

impl SubAssign for ProfanityCensorshipOptions {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self::UseCustomWordSet(self.set().difference(rhs.set()).cloned().collect())
    }
}

impl<S> SubAssign<S> for ProfanityCensorshipOptions
    where
        S: Into<String> {
    fn sub_assign(&mut self, rhs: S) {
        *self = Self::UseCustomWordSet(self.iter().filter(|&s| s != &rhs).collect())
    }
}

impl Add for ProfanityCensorshipOptions {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<S> Add<S> for ProfanityCensorshipOptions
where
    S: Into<String> {
    type Output = Self;

    fn add(mut self, rhs: S) -> Self::Output {
        self += rhs;
        self
    }
}

impl Sub for ProfanityCensorshipOptions {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<S> Sub<S> for ProfanityCensorshipOptions
    where
        S: Into<String> {
    type Output = Self;

    fn sub(mut self, rhs: S) -> Self::Output {
        self -= rhs;
        self
    }
}

fn alias(text: &str) -> String {
    text.chars()
        .map(|character| character_aliases::CHARACTER_ALIASES.get(&character).copied().unwrap_or(character))
        .collect()
}

fn remove_whitespace(text: &str) -> (String, HashMap<usize, usize>) {
    let mut output = String::new();
    let mut hashmap = HashMap::new();

    for (index, (jndex, character)) in text
        .chars()
        .enumerate()
        .filter(|(_, c)| !c.is_whitespace())
        .enumerate() {
        output.push(character);
        hashmap.insert(index, jndex);
    }

    (output, hashmap)
}

fn remove_non_alphabetic(text: &str) -> (String, HashMap<usize, usize>) {
    let mut output = String::new();
    let mut hashmap = HashMap::new();

    for (index, (jndex, character)) in text
        .chars()
        .enumerate()
        .filter(|(_, c)| c.is_alphabetic())
        .enumerate() {
        output.push(character);
        hashmap.insert(index, jndex);
    }

    (output, hashmap)
}

fn deduplicate_string(text: &str) -> (String, HashMap<usize, Vec<usize>>) {
    let mut last = None;
    let mut output = String::new();
    let mut hashmap = HashMap::new();
    let mut jndex = 0;

    for (index, character) in text.chars().enumerate() {
        if last.map(|l| l != character).unwrap_or(true) {
            output.push(character);
            hashmap.entry(jndex).or_insert_with(Vec::new).push(index);
            jndex += 1;
        }
        else {
            hashmap.entry(jndex).or_insert_with(Vec::new).push(index);
        }

        last = Some(character);
    }

    (output, hashmap)
}
