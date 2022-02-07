use crate::List::{Cons, Nil};
fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    point();
    point_2();
}
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn box_new() {
    let b = Box::new(5);
    println!("b = {}", b);
}

fn point() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn point_2() {
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
}
