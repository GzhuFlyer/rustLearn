use std::error::Error;
use std::fs::File;
// 有一些情况 panic 比返回 Result 更为合适，不过他们并不常见。让我们讨论一下为何在示例、
// 代码原型和测试中，以及那些人们认为不会失败而编译器不这么看的情况下， panic 是合适的。
// 章节最后会总结一些在库代码中如何决定是否要 panic 的通用指导原则。
fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    // let f = File::open("hello.txt")?;//error
    let f = File::open("hello.txt")?;
    Ok(())
}
