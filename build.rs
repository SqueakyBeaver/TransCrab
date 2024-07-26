use std::fs;

fn main() {
    let path = fs::canonicalize("libs/vosk").unwrap();
    println!("cargo:rustc-link-search={}", path.to_str().unwrap())
}
