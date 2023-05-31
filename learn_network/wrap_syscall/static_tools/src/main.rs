use std::process::Command;

fn main() {
    println!("Hello, world!");
   
    let mut echo_hello = Command::new("ossutil");
    echo_hello.arg("cp")
              .arg("a.txt")
              .arg("oss://burstfs-dev/10086/10010/tmp/2.txt")
              .arg("--parallel=32");


    let hello_1 = echo_hello.output().expect("failed to execute process").stdout;
    println!("hello_1 = {:?}",std::str::from_utf8(&hello_1).unwrap());
}





// RUSTFLAGS=' -C relocation-model=static -C strip=symbols' cargo build --release --target x86_64-unknown-linux-musl
//  cargo build --release --target x86_64-unknown-linux-musl
// https://stackoverflow.com/questions/61553723/whats-the-difference-between-statically-linked-and-not-a-dynamic-executable
