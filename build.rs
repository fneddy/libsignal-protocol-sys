extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::PathBuf;
//  use cmake;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let dst = cmake::build("libsignal-protocol-c");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-flags=-L {}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=signal-protocol-c");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}/include",out_dir))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
