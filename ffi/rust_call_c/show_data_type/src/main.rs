use libc::c_int;
use libc::c_void;

#[link(name = "testffi")]

extern "C" {
    fn test1(arg1: c_int, arg2: *mut c_void) -> i32;
}
fn main() {
    let a = 123;
    let b = "hello\0".as_ptr() as *mut c_void;
    let ret = unsafe { test1(a, b) };
    println!("ret = {}", ret);
}
