fn main() {
    println!("Hello, world!");
    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    let some_number = Some(5);
    let some_string = Some("a string");
    //     如果使用 None 而不是 Some ，需要告诉 Rust Option<T> 是什么类型的，因为编译器只通过
    // None 值无法推断出 Some 成员保存的值的类型。
    let absent_number: Option<i32> = None;
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y;//error
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
// // 以使用一种更简洁的方式来表达相同的概念，仅仅使用枚举并将数据直接放进每一个枚举
// // 成员而不是将枚举作为结构体的一部分。 IpAddr 枚举的新定义表明了 V4 和 V6 成员都关联了
// // String 值：
// enum IpAddr {
//     V4(String),
//     V6(String),
// }
// 可以将任意类型的数据放入枚举成员中：例如字符串、数字类型或者结构体。甚
// 至可以包含另一个枚举！
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Quit 没有关联任何数据。
// Move 包含一个匿名结构体。
// Write 包含单独一个 String 。
// ChangeColor 包含三个 i32 。
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska, // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
