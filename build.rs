fn main() {
    // required for tango benchmarking, see https://lib.rs/crates/tango-bench
    println!("cargo:rustc-link-arg-benches=-rdynamic");
    println!("cargo:rerun-if-changed=build.rs");
}
