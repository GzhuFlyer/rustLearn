fn main() {
    println!("Hello, world!");
    let mut s = [1, 2, 3];
    let ptr: *mut u8 = s.as_mut_ptr();

    unsafe {
        println!("{:?}", ptr.offset(0));
        println!("{:?}", ptr.offset(1));
        println!("{:?}", ptr.offset(2));
    }
    unsafe {
        println!("{}", *ptr.offset(0));
        println!("{}", *ptr.offset(1));
        println!("{}", *ptr.offset(2));
    }
}
