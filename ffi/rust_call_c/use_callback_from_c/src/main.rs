extern "C" fn callback(a: i32) {
    println!("I'm called from C with value {0}", a);
}

#[link(name = "ext")]
extern "C" {
    fn register_callback(cb: extern "C" fn(i32)) -> i32;
    fn trigger_callback();
}
// 回调都是外部 C 库的直接的函数调用。
//当前线程的控制权从 Rust 转移到 C 再转移回 Rust，
//不过最终回调都是在调用触发回调的函数的线程里执行的

//如果外部库启动了自己的线程，并在那个线程里调用回调函数，情况就变得复杂了。
//这时再访问回调中的 Rust 数据结构是非常不安全的，必须使用正常地同步机制。
//除了 Mutex 等传统的同步机制，还有另一个选项就是使用 channel
//（在 std::sync::mpsc 中）将数据从触发回调的 C 线程传送给一个 Rust 线程。

fn main() {
    unsafe {
        register_callback(callback);
        trigger_callback(); // Triggers the callback.
    }
}
