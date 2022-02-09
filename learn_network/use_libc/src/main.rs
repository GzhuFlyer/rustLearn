use nix::{dir::Dir, fcntl::OFlag, sys::stat::Mode};
use std::{iter::Iterator, string::String};

fn main() {
    unsafe {
        let pid = libc::fork();
        let d = libc::S_IFDIR(12);
        if pid < 0 {
            eprintln!("error!!!");
        } else if pid == 0 {
            println!("子进程空间");
        } else {
            println!("父进程空间，子进程pid为{}", pid);
        }
    }
}

fn ls_upper(dirname: &str) -> impl Iterator<Item = String> {
    let d = Dir::open(dirname, OFlag::O_DIRECTORY, Mode::S_IXUSR).unwrap();
    d.into_iter().map(|x| {
        x.unwrap()
            .file_name()
            .as_ref()
            .to_string_lossy()
            .to_ascii_uppercase()
    })
}
