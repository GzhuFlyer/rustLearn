enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    println!("Hello, world!");
    // 枚举的成员都被定义为相同的枚举类型，所以当需要在
    // vector 中储存不同类型值时，我们可以定义并使用一个枚举！
    // 使用枚举外
    // 加 match 意味着 Rust 能在编译时就保证总是会处理所有可能的情况
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
