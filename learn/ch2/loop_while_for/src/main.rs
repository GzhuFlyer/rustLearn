fn main() {
    println!("Hello, world!");
    // loop_fn();
    while_fn();
}

// 外层循环有一个标签 counting_up ，它将从 0 数到 2。
// 没有标签的内部循环从 10 向下数到 9。
// 第一个没有指定标签的 break 将只退出内层循环。
// break 'counting_up; 语句将退出外层循环
fn loop_fn() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
}

fn loop_fn2() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}

fn while_fn() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");
}
