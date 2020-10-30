use std::{env, path::PathBuf};

extern crate bindgen;

fn main() {
    println!("cargo:rerun-if-changed=src/wrapper.cpp");
    println!("cargo:rerun-if-changed=src/wrapper.hpp");

    cc::Build::new()
        .file("src/wrapper.cpp")
        .include("include/")
        .compile("wrapper");

    let bindings = bindgen::Builder::default()
        .header("src/wrapper.hpp")
        .clang_arg("-Iinclude/")
        .clang_arg("-xc++")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .whitelist_type("lua_State")
        .whitelist_type("Garrysmod.Lua.Type")
        .whitelist_type("wrap_str")
        .whitelist_function("lua_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings!");
}
