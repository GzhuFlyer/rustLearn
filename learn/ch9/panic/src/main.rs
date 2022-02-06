use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
fn main() {
    println!("Hello, world!");
    // panic_dire();
    // panic_vec();
    // file_panic();
    unwrap_panic();
    read_username_from_file();
}

fn panic_dire() {
    panic!("crash and burn");
}

fn panic_vec() {
    // 这种情况下其他像 C 这样语言会尝试直接提供所要求的值，即便这可能不是你期望的：你会得到
    // 任何对应 vector 中这个元素的内存位置的值，甚至是这些内存并不属于 vector 的情况。这被称为
    // 缓冲区溢出（buffer overread），并可能会导致安全漏洞，比如攻击者可以像这样操作索引来读取
    // 储存在数组后面不被允许的数据。
    let v = vec![1, 2, 3];
    v[99];
}
// 匹配不同的错误
fn file_panic() {
    let f = File::open("hello.txt");
    //v1
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error)
    //     }
    // };
    //v2
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error),
    //     },
    // };
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
// 失败时 panic 的简写：unwrap 和 expect
fn unwrap_panic() {
    // 如果调用这段代码时不存在 hello.txt 文件，我们将会看到一个 unwrap 调用 panic! 时提供的错
    // 误信息：
    // let f = File::open("hello.txt").unwrap();
    // 如果在多处使用 unwrap ，则需要花更多的时间来分析到底是哪一个
    // unwrap 造成了 panic，因为所有的 unwrap 调用都打印相同的信息。
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
// 将所有的成功或失败信息向上传播
fn read_username_from_file() -> Result<String, io::Error> {
    // let f = File::open("hello.txt");
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
    //v2
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)
    //v3
    // let mut s = String::new();
    // File::opv4en("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)
    //v4
    fs::read_to_string("hello.txt")
}
