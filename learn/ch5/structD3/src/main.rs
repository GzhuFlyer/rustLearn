#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// &self 实际上是 self: &Self 的缩写。在一个 impl 块中， Self 类型是 impl 块的类型的别名。方法的第一个参数必
// 须有一个名为 self 的 Self 类型的参数，所以 Rust 让你在第一个参数位置上只用 self 这个名
// 字来缩写。注意，我们仍然需要在 self 前面使用 & 来表示这个方法借用了 Self 实例，就像我
// 们在 rectangle: &Rectangle 中做的那样。方法可以选择获得 self 的所有权，或者像我们这
// 里一样不可变地借用 self ，或者可变地借用 self ，就跟其他参数一样。
// 如果想要在方法中改变调用方法的实例，需要将
// 第一个参数改为 &mut self 。通过仅仅使用 self 作为第一个参数来使方法获取实例的所有权是
// 很少见的；这种技术通常用在当方法将 self 转换成别的实例的时候，这时我们想要防止调用者
// 在转换之后使用原始的实例。
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
