//! Raw FFI Caffe module.

#[allow(dead_code, non_camel_case_types, missing_docs,
        missing_copy_implementations)]
mod ffi {
    use ::libc::size_t;
    include!(concat!(env!("OUT_DIR"), "/lib/ffi.rs"));
}

pub use self::ffi::*;
