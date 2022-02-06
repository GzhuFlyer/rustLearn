fn main() {
    print_string();
    push_str();
    format_();
    string_intern();
    get_string();
}
// 遍历字符串的方法
fn get_string() {
    // 分开并返回六个 char 类型的值
    for c in "नमस्ते ".chars() {
        println!("{}", c);
    }
    // bytes 方法返回每一个原始字节
    for b in "नमस्ते ".bytes() {
        println!("{}", b);
    }
}
// String 是一个 Vec<u8> 的封装
fn string_intern() {
    // let s1 = String::from("hello");
    // let h = s1[0];//error
    let len = String::from("Hola").len();
    println!("len is {}", len);
    let len = String::from("Здравствуйте").len();
    println!("len is {}", len);
}
// 使用 + 运算符或 format! 宏拼接字符串
// &String 可以被 强转（coerced）成 &str
fn format_() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    println!("s3 is {}", s3);
    println!("s2 is {}", s2);
    // println!("s1 is {}", s1);
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s);
    println!("s1 is {}", s1);
    println!("s3 is {}", s3);
    println!("s2 is {}", s2);
}
// 使用 push_str 和 push 附加字符串
fn push_str() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
    println!("s1 is {}", s1);
    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s);
}

fn print_string() {
    println!("Hello, world!");
    let mut s = String::new();
    let hello = String::from("عليكم السالم");
    println!("{}", hello);
    let hello = String::from("Dobrý den");
    println!("{}", hello);
    let hello = String::from("Hello");
    println!("{}", hello);
    let hello = String::from("וםֹשלָׁ");
    println!("{}", hello);
    let hello = String::from("नमस्ते ");
    println!("{}", hello);
    let hello = String::from("こんにちは");
    println!("{}", hello);
    let hello = String::from("안녕하세요");
    println!("{}", hello);
    let hello = String::from("你好");
    println!("{}", hello);
    let hello = String::from("Olá");
    println!("{}", hello);
    let hello = String::from("Здравствуйте");
    println!("{}", hello);
    let hello = String::from("Hola");
    println!("{}", hello);
}
