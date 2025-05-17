extern crate bindgen;
extern crate pkg_config;

fn main() {
    use std::{env, path::Path};
    
    // search for gbm header files location
    // used for bindgen generation
    pkg_config::probe_library("gbm").unwrap();
    let gbm_include :String = match pkg_config::get_variable("gbm","includedir") {
        Ok(path) => path,
        Err(_) => panic!("Failed to get gbm include path")
    };
    const TMP_BIND_PREFIX_REG: &str = "_BINDGEN_TMP_.*";
    

    // Setup bindings builder
    let generated = bindgen::builder()
        .clang_arg("-Iinclude")
        .header(format!("{}/gbm.h", gbm_include))
        .blocklist_type(TMP_BIND_PREFIX_REG)
        .ctypes_prefix("libc")
        .allowlist_type("^gbm_.*$")
        .allowlist_function("^gbm_.*$")
        .allowlist_var("GBM_.*|gbm_.*")
        .constified_enum_module("^gbm_.*$")
        // Layout tests are incorrect across architectures
        .layout_tests(false)
        .generate()
        .unwrap();

    println!("cargo:rerun-if-changed={}/gbm.h", gbm_include);

    // Generate the bindings
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("bindings.rs");

    generated.write_to_file(dest_path).unwrap();
    
    {
        use std::fs;

        let bind_file = Path::new(&out_dir).join("bindings.rs");
        let dest_file = "src/bindings.rs";

        fs::copy(bind_file, dest_file).unwrap();
    }
}
