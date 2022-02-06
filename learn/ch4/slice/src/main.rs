fn main() {
    // println!("Hello, world!");
    // let mut t = String::from("hello world!");
    // let word = first_word(&t);
    // // t.clear();
    // println!("{}", word);
    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);
    let my_string_literal = "hello world";
    // first_word 中传入字符串字面值的 slice
    let word = first_word(&my_string_literal);
    // 因为字符串字面值 **就是** 字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
    let word = first_word(my_string_literal);
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{:?}", slice);
    assert_eq!(slice, &[2, 3]);
}
// 编写一个函数，该函数接收一个字符串，并返回在该字符串中找到的第
// 一个单词。如果函数在该字符串中并未找到空格，
// 则整个字符串就是一个单词，所以应该返回整个字符串
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
