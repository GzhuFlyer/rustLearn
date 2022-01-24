fn main() {
    println!("Hello, world!");
    demo3();
}

fn demo1(){
    let number = 3;
    // 不像 Ruby 或 JavaScript 这样的语言，
    // Rust 并不会尝试自动地将非布尔值转换为布尔值。
    // 必须总是显式地使用布尔值作为 if 的条件。
    // if number{   
    if number < 5 {
         println!("condition was true"); 
    } else {
             println!("condition was false");
    }
}

fn demo2(){
    let number = 6;
    if number % 4 == 0 {
         println!("number is divisible by 4"); 
        } else if number % 3 == 0 {
             println!("number is divisible by 3"); 
            } else if number % 2 == 0 {
                println!("number is divisible by 2"); 
            } else { println!("number is not divisible by 4, 3, or 2"); 
        }
}

fn demo3(){
    let condition = true;
     let number = if condition {
          5 
    } else {
         6 
    };
    println!("The value of number is: {}", number);
}