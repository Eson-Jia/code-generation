use std::env;
use std::path::Path;
use std::fs;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("hello.rs");
    fs::write(
        &dest_path,
        "pub fn message() -> &'static str {\
                    \"Hello, World!\"
        }\
        "
    ).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}