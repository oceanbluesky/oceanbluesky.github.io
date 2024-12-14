
// build.rs (used to create 'dist' directory)
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir).join("../../dist");

    // Create the dist directory if it doesn't exist
    if !target_dir.exists() {
        fs::create_dir_all(&target_dir).unwrap();
    }

    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=Dioxus.toml");

    println!("Running build.rs...");
    println!("OUT_DIR: {}", out_dir);
}
