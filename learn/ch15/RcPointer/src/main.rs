enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use crate::List::{Cons, Nil};
use std::rc::Rc;
fn main() {
    show_strong_count_pointer();
}
// 为了启用多所有权，
// Rust 有一个叫做 Rc<T> 的类型。其名称为 引用计数
fn rcP() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // 不过在这里 Rust 的习惯是使用 Rc::clone 。 Rc::clone 的实现并不像大部分类型的 clone 实现那样对所有数据进行深拷贝。 Rc::clone 只
    // 会增加引用计数，这并不会花费多少时间。深拷贝可能会花费很长时间。通过使用 Rc::clone 进
    // 行引用计数，可以明显的区别深拷贝类的克隆和增加引用计数类的克隆。当查找代码中的性能问
    // 题时，只需考虑深拷贝类的克隆而无需考虑 Rc::clone 调用。
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}

fn show_strong_count_pointer() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
