use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // 主线程会等待直到新建线程执行完毕之后才开始执行 for 循环，所以输出将不会交替出现
    // handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    //  handle.join() 调用会等待直到新建线程执行完毕。
         handle.join().unwrap();
}
