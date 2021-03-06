// main.rs
#[link(name = "sorting", kind = "static")]
extern "C" {
    fn interop_sort(arr: *mut i32, n: u32);
}

pub fn sort_from_cpp(arr: &mut [i32]) {
    unsafe {
        // 传入必然有效的数组引用，并通过传入数组的长度来保证不会出现越界访问，从而保证函数内存安全
        interop_sort(arr as *mut [i32] as *mut i32, arr.len() as u32);
    }
}

fn main() {
    let mut my_arr: [i32; 10] = [10, 42, -9, 12, 8, 25, 7, 13, 55, -1];
    println!("Before sorting...");
    println!("{:?}\n", my_arr);

    sort_from_cpp(&mut my_arr);

    println!("After sorting...");
    println!("{:?}\n", my_arr);
}
