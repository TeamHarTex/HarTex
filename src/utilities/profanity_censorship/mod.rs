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
    collections::{
        HashMap
    },
    lazy::{
        SyncLazy
    }
};

static CHARACTER_ALIASES: SyncLazy<HashMap<char, char>> = SyncLazy::new(|| {
    let mut hashmap = HashMap::<char, char>::new();
    const CASE_DIFFERENCE: u8 = b'a' - b'A';

    for character in b'A'..=b'Z' {
        hashmap.insert(character as char, (character + CASE_DIFFERENCE) as char);
    }

    // aliases for the letter A.
    ['4', '@', 'À', 'Á', 'Â', 'Ã', 'Ä', 'Å', 'à', 'á', 'â', 'ã', 'ä', 'å', 'α', 'Α']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 'a');
        });

    // aliases for the letter B.
    ['ß', 'Β', '฿']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 'b');
        });

    // aliases for the letter C.
    ['¢', 'ç', 'Ç', '©']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 'c');
        });

    // aliases for the letter D.
    ['Ð', '₫']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 'd');
        });

    // aliases for the letter E.
    ['3', '£', '€', 'È', 'É', 'Ê', 'Ë', 'è', 'é', 'ê', 'ë', 'ε', 'Ε', 'Ξ', 'Σ']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 'e');
        });

    // aliases for the letter F

    // aliases for the letter G
    ['6']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 'g');
        });

    // aliases for the letter H
    ['Η']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 'h');
        });

    // aliases for the letter I
    ['1', '|', '!', 'Ì', 'Í', 'Î', 'Ï', 'ì', 'í', 'î', 'ï', 'Ι']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 'i');
        });

    // aliases for the letter J

    // aliases for the letter K
    ['κ', 'Κ']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 'k');
        });

    // aliases for the letter L

    // aliases for the letter M
    ['Μ']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 'm');
        });

    // aliases for the letter N
    ['ñ', 'Ñ', 'η', 'Ν', 'Π']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 'n');
        });

    // aliases for the letter O
    ['0', 'Ò', 'Ó', 'Ô', 'Õ', 'Ö', 'ò', 'ó', 'ô', 'õ', 'ö', 'Ø', 'ø', 'θ', 'ο', 'σ', 'Θ', 'Ο', 'Φ']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 'o');
        });

    // aliases for the letter P
    ['ρ', 'Ρ', '₱', '℗', 'Þ', 'þ']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 'p');
        });

    // aliases for the letter Q

    // aliases for the letter R
    ['®']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 'r');
        });

    // aliases for the letter S
    ['5', '$']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 's');
        });

    // aliases for the letter T
    ['τ', 'Τ']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 't');
        });

    // aliases for the letter U
    ['Ù', 'Ú', 'Û', 'Ü', 'ù', 'ú', 'û', 'ü', 'μ', 'υ']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 'u');
        });

    // aliases for the letter V
    ['ν']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 'v');
        });

    // aliases for the letter W
    ['ω', '₩']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 'w');
        });

    // aliases for the letter X
    ['×', 'χ', 'Χ']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 'x');
        });

    // aliases for the letter Y
    ['¥', 'Ý', 'ý', 'ÿ', 'γ', 'Υ']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 'y');
        });

    // aliases for the letter Z
    ['2', 'Ζ']
        .iter()
        .for_each(|character| {
            hashmap.insert(*character, 'z');
        });

    hashmap
});
