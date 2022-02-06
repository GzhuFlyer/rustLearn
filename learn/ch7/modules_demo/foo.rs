// modules_demo/foo.rs
// 使用了 mod 声明了模块 bar。接下来，
// 从模块 bar 中重新导出 Bar。
// 重新导出适合导入隐藏在嵌套子模块中的项。
// 可以看到，pub use 指明重新导出的 Bar 并不是在这里实现的。
// pub use 就是把其它地方的元素当作模块的直接成员公开出去。
// 有了这个机制，就可以轻松做好接口和实现的分离。

mod bar;
pub use self::bar::Bar;

pub fn do_foo() {
  println!("Hi from foo!");
}
