//! # The `util` Module
//!
//! This module defines several utilites for reading DSL source.

pub(in crate::read) enum Ref<'borrowed, 'copied, T>
where
    T: ?Sized + 'static {
    Borrowed(&'borrowed T),
    Copied(&'copied T)
}
