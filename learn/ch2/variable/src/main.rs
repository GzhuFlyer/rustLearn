use std::io;
//常量
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; 
fn main() {
    // shadowing();
    // unit_type();
    // array();
    give_num();
}



fn give_num(){
    let x = 5;
    let y = { let x = 3; x + 1 };
    println!("The value of y is: {}", y);
}

// Rust 安全原则的例子。在很多底层语言中，并没有进行这类检查，这
// 样当提供了一个不正确的索引时，就会访问无效的内存。
fn array(){
    let a = [1, 2, 3, 4, 5];
     println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin() .read_line(&mut index) .expect("Failed to read line");
    let index: usize = index .trim() .parse() .expect("Index entered was not a number");
    let element = a[index];
     println!( "The value of the element at index {} is: {}", index, element );
}

//元组类型
//为了从元组中获取单个值，可以
//使用模式匹配（pattern matching）来解构（destructure）元组值
fn unit_type(){
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y)
}

//基本数据类型
fn basic_struct(){
    //float
    let x = 2.0; // f64 
    let y: f32 = 3.0; // f32
    let sum = 5 + 10; // 减法
    let difference = 95.5 - 4.3; // 乘法
    let product = 4 * 30; // 除法
    let quotient = 56.7 / 32.2; 
    let floored = 2 / 3; // 结果为 0 // 取余
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z = 'ℤ';    
    let heart_eyed_cat = '😻';  //字符类型

}

fn shadowing(){
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}

fn variable(){
    //变量默认是不可改变,加 mut 就可以了。
    let mut x = 5;
    println!("The value of x is: {}",x);
    x = 6;
    println!("The value of x is: {}",x);
}