use std::env;
use std::path::Path;
use std::process::Command;
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("cc")
        .args(&["hello-world/hello.c", "-O3", "-c", "-fPIC", "-o"])
        .arg(&format!("{}/hello-world.o", out_dir))
        .status()
        .unwrap();

    Command::new("ar")
        .args(&["crus", "libhello-world.a", "hello-world.o"])
        .current_dir(&Path::new(&out_dir))
        .status()
        .unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=hello-world");
}
