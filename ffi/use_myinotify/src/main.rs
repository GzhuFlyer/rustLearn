use std::{
    ffi::CString,
    io,
    os::unix::ffi::OsStrExt,
    os::unix::io::{
        AsRawFd,
        FromRawFd,
        IntoRawFd,
        RawFd,
    },
    path::Path,
    sync::{
        atomic::AtomicBool,
        Arc,
    }
};

use myinotify as ffi;
use libc::{
    F_GETFL,
    F_SETFD,
    F_SETFL,
    FD_CLOEXEC,
    O_NONBLOCK,
    fcntl,
};

fn main() {
    println!("Hello, world!");
    let fd = unsafe {
        let fd = ffi::inotify_init();
        if fd == -1 {
            // return Err(io::Error::last_os_error());
        }
        if fcntl(fd, F_SETFD, FD_CLOEXEC) == -1 {
            // return Err(io::Error::last_os_error());
        }
        if fcntl(fd, F_SETFL, O_NONBLOCK) == -1 {
            // return Err(io::Error::last_os_error());
        }
    let path = std::path::Path::new("/home/fzw/workspace/bky/use_rustlen/rustLearn/ffi/use_myinotify");
    let path = CString::new(path.as_os_str().as_bytes()).unwrap();
    let wd = 
        ffi::inotify_add_watch(
           fd,
            path.as_ptr() as *const _,
            ffi::IN_CREATE,
        );
        println!("wd = {:?}",wd);
        println!("fd = {}", fd);
        libc::sleep(1000);
    };
    
}
