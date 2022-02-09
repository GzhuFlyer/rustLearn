fn main() {
    c();
}

fn _a() {
    let cmd_line = std::env::args();
    println!("总共有 {} 个命令行参数", cmd_line.len()); // 传递的参数个数
    for arg in cmd_line {
        println!("[{}]", arg); // 迭代输出命令行传递的参数
    }
}
fn b() {
    let mut line = String::new();
    println!("请输入你的名字:");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("你好 , {}", line);
    println!("读取的字节数为：{}", b1);
}
fn c() {
    let cmd_line = std::env::args();
    println!("总共有 {} 个命令行参数", cmd_line.len()); // 传递的参数个数
    let mut sum = 0;
    let mut has_read_first_arg = false;
    //迭代所有参数并计算它们的总和
    for arg in cmd_line {
        if has_read_first_arg {
            // 跳过第一个参数，因为它的值是程序名
            sum += arg.parse::<i32>().unwrap();
        }
        has_read_first_arg = true; // 设置跳过第一个参数，这样接下来的参数都可以用于计算
    }
    println!("和值为:{}", sum);
}
