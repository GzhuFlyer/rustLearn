fn main() {
    println!("Hello, world!");


    let a = test();
    println!(" a = {}\n\n", a);

    let guard = match InterceptGuard::try_lock() {
        Some(g) => {println!("g11 = {:?}",g);g},
        None =>{ println!("Syscall::ForwardToKernel as i32 {}", Syscall::ForwardToKernel as i32);return;},//save_current_syscall_info
    };
    
    // let a = test1();
    // println!(" a = {}", a);
}


fn test() -> i32{
    let guard = match InterceptGuard::try_lock() {
        Some(g) => {println!("g = {:?}",g);g},
        None => return Syscall::ForwardToKernel as i32, //save_current_syscall_info
    };
    println!("guard = {:?}",guard );
    // drop(guard);
    let guard = match InterceptGuard::try_lock() {
        Some(g) => g,
        None => return Syscall::ForwardToKernel as i32, //save_current_syscall_info
    };
    // println!("guard = {:?}",guard );
    2
}

fn test1() -> i32{
    let guard = match InterceptGuard::try_lock() {
        Some(g) => g,
        None => return Syscall::ForwardToKernel as i32, //save_current_syscall_info
    };
    println!("guard = {:?}",guard );
    let guard = match InterceptGuard::try_lock() {
        Some(g) => g,
        None => return Syscall::ForwardToKernel as i32, //save_current_syscall_info
    };
    println!("guard = {:?}",guard );
    2
}

pub enum Syscall {
    Hooked = 0,
    ForwardToKernel = 1,
}
use std::cell::Cell;
thread_local! {
    /// A flag indicating whether the current thread is in an intercept context.
    static INTERCEPTED: Cell<bool> = Cell::new(false);
}

#[derive(Debug)]
struct InterceptGuard;

impl InterceptGuard {
    fn try_lock() -> Option<Self> {
        INTERCEPTED.with(|x| {
            if x.get() {
                None
            } else {
                x.set(true);
                Some(InterceptGuard)
            }
        })
    }
}

impl Drop for InterceptGuard {
    fn drop(&mut self) {
        INTERCEPTED.with(|x| x.set(false));
    }
}