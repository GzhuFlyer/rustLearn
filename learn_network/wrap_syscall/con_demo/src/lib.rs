// use libc_print::std_name::println;
// use crate::ctor;

// #[ctor]
// fn foo() {
//   println!("Hello, world!");
// }
#[macro_use]
extern crate ctor;

// use libc_print::*;
// use std::path::Path;
// use std::process::Command;
// use std::sync::atomic::{AtomicBool, Ordering};

// static INITED: AtomicBool = AtomicBool::new(false);
// static INITED_2: AtomicBool = AtomicBool::new(false);

/// Doc comment
#[ctor]
fn foo() {
    // INITED.store(true, Ordering::SeqCst);
    println!("hello " );
}
