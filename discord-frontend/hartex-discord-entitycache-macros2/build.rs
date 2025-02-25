#![feature(rustc_private)]

extern crate rustc_driver;

pub fn main() {
    hartex_reflect::reflect_crate("twilight-model");
}
