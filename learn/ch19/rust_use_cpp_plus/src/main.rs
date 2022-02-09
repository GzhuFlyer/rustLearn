use std::ffi::CString;
#[link(name = "hello-world")]
extern "C" {
    fn greet(s: *const std::os::raw::c_char, a: i32, b: i32);
}
fn main() {
    let prompt = CString::new("Rust").unwrap();
    unsafe {
        greet(prompt.as_ptr(), 7, 11);
    }
}
