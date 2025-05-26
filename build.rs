use std::{env, path::PathBuf};

fn main() {
    let bindings = bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("::core::ffi")
        .header("ppcd.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    cc::Build::new().file("ppcd.cpp").compile("ppcd");
}
