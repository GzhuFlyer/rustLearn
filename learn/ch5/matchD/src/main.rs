fn main() {
    println!("Hello, world!");

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), //如果你掷出 3 或 7 以外的值，你的回合将无事发生。
    }

    let some_u8_value = Some(0u8);
    // match some_u8_value {
    //     Some(3) => println!("three"),
    //     _ => (),
    // }
    // if let 是 match 的一个语法糖，
    // 它当值匹配某一模式时执行代码而忽略所有其他值
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
