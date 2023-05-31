include!("../bindings.rs");

pub unsafe fn unset_hook_fn() {
    intercept_hook_point = None;
}
pub unsafe fn set_hook_fn(f: HookFn) {
    intercept_hook_point = Some(f);
}
pub type HookFn = extern "C" fn(
    num: i64,
    a0: i64,
    a1: i64,
    a2: i64,
    a3: i64,
    a4: i64,
    a5: i64,
    result: *mut i64,
) -> i32;

#[macro_use]
extern crate ctor;
#[ctor]
fn foo() {
    println!("hello " );
    unsafe { set_hook_fn(hook) };

}
extern "C" fn hook(
    num: i64,
    _a0: i64,
    _a1: i64,
    _a2: i64,
    _a3: i64,
    _a4: i64,
    _a5: i64,
    result: *mut i64,
) -> i32 {
    use libc::*;
   if num == SYS_getdents{
    unsafe {
        *result = -ENOTSUP as  i64;
    } 
    0
   }else{
    1
   }
}

#[cfg(test)]
mod tests{

}