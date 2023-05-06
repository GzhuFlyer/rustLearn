// fn main() {
//     println!("Hello, world!");
//     // let c: u64 = 18446744073709551615;
//     // // println!("b = {0x?}",b);
//     // //              3458764513820540927
//     // let a:u64 = 0x2fffffffffffffff;
//     // let b:isize = a.try_into().unwrap();
//     // println!("b = {:?}",b);
//     // let a :isize = 0;
//     // println!("a change = {:#x?}",(-a as usize));
//     // let b: usize = 2;
//     // // let c:u64 = b  + a as u64;
//     // let c:usize = match a>0 {
//     //     true => {b + a as usize}
//     //     false =>{ b - (-a as usize)}
//     // };
//     // println!("c = {:?}",c);
//     // let a :usize = 0;
//     // let b :i32 = -1;
//     // if (a as i32) < b{
//     //     println!("hello");
//     // }
//     let a :isize = -1;
//     println!("a = {:?}",a as usize);
//     let b = usize::MAX;
//     println!("b = {:?}",b);
// }
use tokio::runtime::Runtime;

fn function_that_spawns(msg: String) {
    // Had we not used `rt.enter` below, this would panic.
    tokio::spawn(async move {
        println!("{}", msg);
    });
    println!("come to here..");
}

fn main() {
    let rt = Runtime::new().unwrap();

    let s = "Hello World!".to_string();

    // By entering the context, we tie `tokio::spawn` to this executor.
    let _guard = rt.enter();
    drop(rt);
    function_that_spawns(s);
    println!("good morning!");
}