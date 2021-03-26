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
    ops::{
        Div,
        Rem
    }
};

crate trait IntegerDivRemSimultaneously: Sized + PartialOrd + Ord + Eq + Div + Rem {
    fn div_rem(&self, other: &Self) -> (<Self as Div>::Output, <Self as Rem>::Output) {
        (*self / *other, *self % *other)
    }
}

impl IntegerDivRemSimultaneously for i8 {}

impl IntegerDivRemSimultaneously for i16 {}

impl IntegerDivRemSimultaneously for i32 {}

impl IntegerDivRemSimultaneously for i64 {}

impl IntegerDivRemSimultaneously for i128 {}

impl IntegerDivRemSimultaneously for isize {}

impl IntegerDivRemSimultaneously for u8 {}

impl IntegerDivRemSimultaneously for u16 {}

impl IntegerDivRemSimultaneously for u32 {}

impl IntegerDivRemSimultaneously for u64 {}

impl IntegerDivRemSimultaneously for u128 {}

impl IntegerDivRemSimultaneously for usize {}
