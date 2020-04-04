use std::env;
use std::path::PathBuf;

fn metamod_bindings() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.hpp")
        .clang_arg("-Icpp_stubb_example/dependencies/metamod_p/metamod")
        .clang_arg("-Icpp_stubb_example/dependencies/hlsdk/dlls")
        .clang_arg("-Icpp_stubb_example/dependencies/hlsdk/engine")
        .clang_arg("-Icpp_stubb_example/dependencies/hlsdk/common")
        .clang_arg("-Icpp_stubb_example/dependencies/hlsdk/public")
        .clang_arg("-Icpp_stubb_example/dependencies/metamod_p/metamod")
        .blacklist_type("FP_INT_UPWARD")
        .blacklist_type("FP_INT_DOWNWARD")
        .blacklist_type("FP_INT_TOWARDZERO")
        .blacklist_type("FP_INT_TONEARESTFROMZERO")
        .blacklist_type("FP_INT_TONEAREST")
        .blacklist_type("FP_NAN")
        .blacklist_type("FP_INFINITE")
        .blacklist_type("FP_ZERO")
        .blacklist_type("FP_SUBNORMAL")
        .blacklist_type("FP_NORMAL")
        .blacklist_type("k_GIDNil")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    metamod_bindings()
}
