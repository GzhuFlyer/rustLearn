struct CustomSmartPointer {
    data: String,
}
// 对于智能指针模式来说第二个重要的 trait 是 Drop ，其允许我们在值要离开作用域时执行一些代
// 码。可以为任何类型提供 Drop trait 的实现，同时所指定的代码被用于释放类似于文件或网络连
// 接的资源。
//类似于析构函数
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
fn main() {
    println!("Hello, world!");
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    // c.drop();//error
    drop(c);
    // 强制运行 drop 方法来释放锁以便作用域中的其他代码可以获
    // 取锁。
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
