use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    //     第一行展示了一个函数定义，而第二行展示了一个完整标注的闭包定义。第三行闭包定义中省略
    // 了类型注解，而第四行去掉了可选的大括号，
    //因为闭包体只有一行。这些都是有效的闭包定义，
    // 并在调用时产生相同的行为。
    // fn add_one_v1 (x: u32) -> u32 { x + 1 } //该场景可替换为闭包实现
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x| x + 1;
    // let add_one_v4 = |x| x + 1;
    // let example_closure = |x| x;
    // let s = example_closure(String::from("hello"));
    // let n = example_closure(5);
    let expensive_closure = |num| {
        println!("calculating slowly...");
        // 闭包定义是 expensive_closure 赋值的 = 之后的部分。
        // 闭包的定义以一对竖线（ | ）开始，在竖线中指定闭包的参数；
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
    println!("....");
}
