// build.rs
// Rust 2018 不需要 extern crate 语句

fn main() {
    cc::Build::new()
        .cpp(true)
        .warnings(true)
        .flag("-Wall")
        .flag("-std=c++11")
        .flag("-c")
        .file("intercept_wrap/wrap.cpp")
        .compile("interceptwrap"); // 
    println!("cargo:rerun-if-changed=intercept_wrap/wrap.cpp");
}
