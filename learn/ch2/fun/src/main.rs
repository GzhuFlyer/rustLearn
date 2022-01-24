fn main() {
    println!("Hello, world!");
    return_fn();
}

//具有返回值的函数
fn five() -> i32 { 5 }
fn plus_one(x: i32) -> i32 { x + 1 }
fn return_fn() { 
    let x = five();
    println!("The value of x is: {}", x);
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}