// // 仅当目标系统为Linux 的时候才会编译
// #[cfg(target_os = "linux")]
// fn are_you_on_linux() {
//     println!("linux!")
// }

// // 仅当目标系统不是Linux 时才会编译
// #[cfg(not(target_os = "linux"))]
// fn are_you_on_linux() {
//     println!("not linux!")
// }

// fn main() {
//     are_you_on_linux();
//     println!("Are you sure?");
//     if cfg!(target_os = "linux") {
//         println!("Yes. It's linux!");
//     } else {
//         println!("Yes. It's not linux!");
//     }
// }

//main.rs

// fn conditional_function() {
//     println!("condition met!");
// }

// fn main() {
//     #[cfg(feature = "pjdtest")]
//     {
//         conditional_function();
//     }



//     if cfg!(feature = "pjdtest"){
//         println!("config");
//     }else{
//         println!("not config");
//     }

//     println!("Hello, world!");
// }

// 这是一个简单的宏，名为 `say_hello`。
macro_rules! say_hello {
    // `()` 表示此宏不接受任何参数。
    () => (
        // 此宏将会展开成这个代码块里面的内容。
        println!("Hello!");
    )
}

fn main() {
    // 这个调用将会展开成 `println("Hello");`!
    say_hello!()
}
