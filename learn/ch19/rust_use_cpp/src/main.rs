extern crate libc;

extern "C" {
    fn triple_input(input: libc::c_int) -> libc::c_int;
    // fn hello_world();
}

fn main() {
    // hello_world();
    let input = 4;
    let output = unsafe { triple_input(input) };
    println!("{} * 3 = {}", input, output);
}
