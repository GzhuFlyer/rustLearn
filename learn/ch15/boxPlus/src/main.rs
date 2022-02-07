use std::ops::Deref;
fn main() {
    println!("Hello, world!");
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
// 为了启用 * 运算符的解引用
// 功能，需要实现 Deref trait
// deref 方法体中写入了 &self.0 ，
//这样 deref 返回了我希望通过 * 运算符访问的值的引用。
// 没有 Deref trait 的话，编译器只会解引用 & 引用类型。
//  deref 方法向编译器提供了获取任何实
// 现了 Deref trait 的类型的值，并且调用这个类型的
// deref 方法来获取一个它知道如何解引用的
// & 引用的能力。
// deref 方法返回值的引用，以及 *(y.deref()) 括号外边的普通解引用仍为必须的原因在于所有
// 权。如果 deref 方法直接返回值而不是值的引用，其值（的所有权）将被移出 self 。在这里以
// 及大部分使用解引用运算符的情况下我们并不希望获取 MyBox<T> 内部值的所有权。
// 注意，每次当我们在代码中使用 * 时， * 运算符都被替换成了先调用 deref 方法再接着使用
// * 解引用的操作，且只会发生一次，不会对 * 操作符无限递归替换
struct MyBox<T>(T);
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
// impl<T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }
fn hello(name: &str) {
    println!("Hello, {}!", name);
}
