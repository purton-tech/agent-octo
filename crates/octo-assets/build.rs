use cache_busters::generate_static_files_code;
use std::env;
use std::path::PathBuf;

fn main() {
    let static_out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let asset_dirs = vec![manifest_dir.join("images"), manifest_dir.join("dist")];

    generate_static_files_code(&static_out_dir, &asset_dirs, &[]).unwrap();
}
