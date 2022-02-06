fn main() {
    println!("Hello, world!");
    let user1 = User {
        email: String::from("someone@exampl"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // user1.email = String::from("another@example.com");
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    println!("{:?}", user1);
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    //     在这个例子中，我们在创建 user2 后不能再使用 user1 ，因
    // 为 user1 的 username 字段中的 String 被移到 user2 中。如果我们给 user2 的 email 和 username 都赋予新的 String 值，从而只使用 user1 的 active 和 sign_in_count 值，那么
    // user1 在创建 user2 后仍然有效。 active 和 sign_in_count 的类型是实现 Copy trait 的类
    // 型，所以我们在“变量与数据交互的方式（二）：克隆” 部分讨论的行为同样适用。
    // let user3 = User {
    //     email: String::from("another@example.com"),
    //     ..user1
    // };
    // 一个获取 Color 类型参数的
    // 函数不能接受 Point 作为参数，即便这两个类型都由三个 i32 值组成。
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;
}
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
// 使用没有命名字段的元组结构体来创建不同的类型
// ，一个获取 Color 类型参数的
// 函数不能接受 Point 作为参数，即便这两个类型都由三个 i32 值组成。
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 没有任何字段的类单元结构体
struct AlwaysEqual;
