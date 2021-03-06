extern crate bindgen;
extern crate run_script;

use std::env;
use std::path::PathBuf;
use std::process::Command;
use run_script::ScriptOptions;

fn main() {
    println!("cargo:rustc-link-search=all=/root/cortx-motr/motr/.libs/");
    println!("cargo:rustc-link-search=all=/root/messy/rust-mio/src");
    println!("cargo:rustc-link-lib=motr");
    println!("cargo:rustc-link-lib=mio");
    println!("cargo:rustc-env=LD_LIBRARY_PATH={}", "/root/cortx-motr/motr/.libs/:/root/messy/rust-mio/src");



    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I/usr/include/motr")
        .clang_arg("-I/root/cortx-motr")
        .clang_arg("-I/root/cortx-motr/extra-libs/galois/include")
        .clang_arg("-DM0_EXTERN=extern")
        .clang_arg("-DM0_INTERNAL=")
        .clang_arg("-Wno-attributes")
        .clang_arg("-L/root/cortx-motr/motr/.libs -Wl,-rpath=/root/cortx-motr/motr/.libs/")
        .trust_clang_mangling(false)
        // -lmotr")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let options = ScriptOptions::new();

    let bindings_rs_path = (out_path.join("bindings.rs").into_os_string().into_string().unwrap());

    let args = vec![bindings_rs_path];

    run_script::run_script!(
        r#"
        sed -i '/pub\sfn\sobject_read()/i \
        }\
        #[link(name = "mio")]\
        #[link(name = "motr")]\
        extern "C" {\
            ' $1
        exit 0
         "#,
        &args,
        &options
    );


}
