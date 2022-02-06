fn main() {
    println!("Hello, world!");
    strings_clone_fn();
    dangle();
}

// 。在 let s2 = s1 之后，
// Rust 认为 s1 不再有效，
// 因此 Rust 不需要在 s1 离开作用域后清理任何东西。
fn strings_fn() {
    let s1 = String::from("hello");
    let s2 = s1;
    //编译时候可以捕获
    // println!("{}, world!", s1);
}

fn strings_clone_fn() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}
// rust会避免悬垂引用，下面代码会出错
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
fn dangle() -> String {
    let s = String::from("hello");
    s
}
