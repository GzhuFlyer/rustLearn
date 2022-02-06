#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect1)
    );
    println!("rect1 is {:#?}", rect1);

    //     因为 dbg! 返回表达式的值的所有权，所以
    // width 字段将获得相同的值，就像我们在那里没有 dbg! 调用一样。我们不希望 dbg! 拥有
    // rect1 的所有权，所以我们在下一次调用 dbg! 时传递一个引用。
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
    dbg!(&rect1);
    println!("{:?}", rect1);
    println!("{:?}", rect1);
}
fn area(width: u32, height: u32) -> u32 {
    width * height
}
fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
