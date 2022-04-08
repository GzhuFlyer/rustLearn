use std::env;
fn main() {
    println!("Hello, world!");
    // for (key, value) in env::vars() {
    //     println!("{} => {}", key, value);
    // }

    let key = "good";
    // match env::var(key) {
    //     Ok(val) => println!("{}: {:?}", key, val),
    //     Err(e) => println!("couldn't interpret {}: {}", key, e),
    // }
    let t = env::var(key);
    if t.is_ok() {
        // println!("value={}", value);
        println!("value={}", t.unwrap());
    } else {
        println!("nothing");
    }
    let c: usize = 0xdeadc0de;
    println!("===={}", c);
}
