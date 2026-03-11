use cache_busters::generate_static_files_code;
use std::env;
use std::path::PathBuf;

fn main() {
    // `cache_busters::generate_static_files_code` does not emit Cargo rebuild
    // directives, so we must watch the asset directories explicitly.
    // VERIFY THIS
    println!("cargo:rerun-if-changed=./images");
    println!("cargo:rerun-if-changed=./dist");

    let static_out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let asset_dirs = vec![PathBuf::from("./images"), PathBuf::from("./dist")];

    generate_static_files_code(&static_out_dir, &asset_dirs, &[]).unwrap();
}
