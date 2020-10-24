use std::env::var;

pub fn link_csfml(lib_name: &str) {
    println!(
        "cargo:rustc-link-search=native={}\\..\\..\\..\\..\\..\\sfml_engine\\sfml-build\\lib",
        var("OUT_DIR").unwrap()
    );
    println!("cargo:rustc-link-lib=csfml-{}", lib_name);
}
