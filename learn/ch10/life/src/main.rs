// fn longest(x: &str, y: &str) -> &str {   //不确定x,y的生命周期
//longest 函数定义指定了签名中所有的引用必须有相同的生命周期 'a
// longest 函数并不需要知道 x 和 y 具体会存在多久，而只需要知道有某个可
// 以被 'a 替代的作用域将会满足这个签名
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// fn main() {
//     println!("Hello, world!");
//     {
//         let r;
//         // ---------+-- 'a // |
//         {
//             // |
//             let x = 5; // -+-- 'b |
//             r = &x; // | |
//         } // -+ | // |
//         println!("r: {}", r); // |
//     }
// }
