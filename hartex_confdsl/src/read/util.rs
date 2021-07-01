//! # The `util` Module
//!
//! This module defines several utilites for reading DSL source.

use std::ops;

pub(in crate::read) enum Ref<'borrowed, 'copied, T>
where
    T: ?Sized + 'static {
    Borrowed(&'borrowed T),
    Copied(&'copied T)
}

impl<'borrowed, 'copied, T> ops::Deref for Ref<'borrowed, 'copied, T>
where
    T: ?Sized + 'static {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match *self {
            Self::Borrowed(t) => t,
            Self::Copied(t) => t
        }
    }
}
