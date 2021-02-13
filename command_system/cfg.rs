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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
crate enum CaseSensitivity {
    False = 0,
    True = 1,
}


#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
crate enum UseFullyQualifiedName {
    False = 0,
    True = 1,
}



#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u8)]
crate enum EnabledAliases {
    False = 0,
    True = 1,
}

crate use CaseSensitivity::{
    True as CaseSensitive,
    False as CaseInsensitive
};


crate use EnabledAliases::{
    True as EnableAliases,
    False as DisableAliases
};

crate use UseFullyQualifiedName::{
    True as FullyQualifiedName,
    False as NoFullyQualifiedName
};
