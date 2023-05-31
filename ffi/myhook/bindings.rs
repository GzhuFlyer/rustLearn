/* automatically generated by rust-bindgen 0.64.0 */

extern "C" {
    pub static mut intercept_hook_point: ::std::option::Option<
        unsafe extern "C" fn(
            syscall_number: ::std::os::raw::c_long,
            arg0: ::std::os::raw::c_long,
            arg1: ::std::os::raw::c_long,
            arg2: ::std::os::raw::c_long,
            arg3: ::std::os::raw::c_long,
            arg4: ::std::os::raw::c_long,
            arg5: ::std::os::raw::c_long,
            result: *mut ::std::os::raw::c_long,
        ) -> ::std::os::raw::c_int,
    >;
}
extern "C" {
    pub static mut intercept_hook_point_clone_child: ::std::option::Option<
        unsafe extern "C" fn(
            flags: ::std::os::raw::c_ulong,
            child_stack: *mut ::std::os::raw::c_void,
            ptid: *mut ::std::os::raw::c_int,
            ctid: *mut ::std::os::raw::c_int,
            newtls: ::std::os::raw::c_long,
        ),
    >;
}
extern "C" {
    pub static mut intercept_hook_point_clone_parent: ::std::option::Option<
        unsafe extern "C" fn(
            flags: ::std::os::raw::c_ulong,
            child_stack: *mut ::std::os::raw::c_void,
            ptid: *mut ::std::os::raw::c_int,
            ctid: *mut ::std::os::raw::c_int,
            newtls: ::std::os::raw::c_long,
            returned_pid: ::std::os::raw::c_long,
        ),
    >;
}
extern "C" {
    pub static mut intercept_hook_point_post_kernel: ::std::option::Option<
        unsafe extern "C" fn(
            syscall_number: ::std::os::raw::c_long,
            arg0: ::std::os::raw::c_long,
            arg1: ::std::os::raw::c_long,
            arg2: ::std::os::raw::c_long,
            arg3: ::std::os::raw::c_long,
            arg4: ::std::os::raw::c_long,
            arg5: ::std::os::raw::c_long,
            result: ::std::os::raw::c_long,
        ),
    >;
}
extern "C" {
    pub fn syscall_no_intercept(
        syscall_number: ::std::os::raw::c_long,
        ...
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn syscall_hook_in_process_allowed() -> ::std::os::raw::c_int;
}