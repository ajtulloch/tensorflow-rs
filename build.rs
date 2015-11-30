// use std::env;
// fn main() {
//     let tf_dir = env::var("TF_ROOT").unwrap();
// println!("cargo:rustc-link-search=native={}/.build_release/lib",
// tf_dir);
// }

extern crate bindgen;

// use std::process::Command;
use std::env;
use std::path::Path;
use std::fs;


// use bindgen::{Bindings, BindgenOptions, LinkType, Logger};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let lib_dir = Path::new(&out_dir).join("lib/");
    let dest_path = Path::new(&lib_dir).join("ffi.rs");

    let _ = fs::create_dir(&lib_dir);

    let mut bindings = bindgen::builder();
    bindings.forbid_unknown_types();

    // //environment specific
    // let tf_root = env::var("TENSORFLOW_ROOT").unwrap();
    let tf_root = String::from("/Users/tulloch/src/tensorflow/");
    let tf_header = Path::new(&tf_root).join("tensorflow/core/public/tensor_c_api.h");
    let tf_include = Path::new(&tf_root).join("include/");
    let tf_lib_dir = Path::new(&tf_root).join(".build_release/lib/");

    bindings.link("tensorflow");
    bindings.match_pat("tensor_c_api.h");
    bindings.header(tf_header.to_str().unwrap());
    // bindings.clang_arg(format!("-I{} -DCPU_ONLY", tf_include.to_str().unwrap()));

    let bindings = bindings.generate();
    let bindings = bindings.unwrap();
    bindings.write_to_file(&dest_path).unwrap();

    println!("cargo:include={}", dest_path.to_str().unwrap());
    println!("cargo:rustc-link-search=native={}", tf_lib_dir.to_str().unwrap());
}
