//! # Module `static_assert`
//!
//! This module contains macros for static assertions.

/// # Macro `static_assert_impl_all`
///
/// This macro asserts that an object implements all the listed traits.
pub macro static_assert_impl_all {
    (type $type:ty: traits $($trait:path),+ $(,)?) => {
        const _: fn() = || {
            fn static_assert_impl_all<T: ?Sized $(+ $trait)+>() {}
            static_assert_impl_all<$type>();
        };
    }
}
