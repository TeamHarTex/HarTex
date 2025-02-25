#![feature(rustc_private)]

extern crate rustc_driver;

pub fn main() {
    #[cfg(not(clippy))]
    hartex_reflect::reflect_crate("twilight-model");
}
