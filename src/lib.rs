//! tensorflow-rs
#![feature(plugin)]
#![feature(unique)]
#![deny(missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unused_import_braces,
        unused_qualifications)]

extern crate libc;

mod tf;

pub mod ffi;
pub use tf::*;
