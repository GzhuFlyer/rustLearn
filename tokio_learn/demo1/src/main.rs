use futures::executor::block_on;
extern crate time;
use time::*;
use tokio::runtime::Builder;
use tokio::time::{sleep, Duration};

fn main() {
    let runtime = Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();

    let mut handles = Vec::with_capacity(10);
    for i in 0..10 {
        handles.push(runtime.spawn(my_bg_task(i)));
        // handles.push(runtime.spawn(my_bg_task(i)));
        // runtime.spawn(my_bg_task(i));
    }
    // for i in 0..10 {
    //     runtime.block_on(my_bg_task(i));
    // }

    // 在后台任务运行的同时做一些耗费时``                                                                                                                                                                                                                                                                   间的事情
    std::thread::sleep(Duration::from_millis(750));
    println!("Finished time-consuming task.");

    // 等待这些后台任务的完成
    for handle in handles {
        // `spawn` 方法返回一个 `JoinHandle`，它是一个 `Future`，因此可以通过  `block_on` 来等待它完成
        // runtime.block_on(handle).unwrap();
        block_on(handle).unwrap();
    }
}

async fn test(i: u64) {
    let now = time::now();
    // let f_now = time::strftime("%Y-%m-%dT%H:%M:%S", &now).unwrap();
    // println!("{} -now: {:?}", i, now.tm_nsec);
    sleep(Duration::from_millis(i)).await;
    let now = time::now();
    // let f_now = time::strftime("%Y-%m-%dT%H:%M:%S", &now).unwrap();
    // println!("{} -now: {:?}", i, now.tm_nsec);
    println!("test {}", i);
}

async fn my_bg_task(i: u64) {
    let millis = 1000 - 50 * i;
    println!("Task {} sleeping for {} ms.", i, millis);

    // sleep(Duration::from_millis(millis)).await;
    test(millis).await;

    println!("Task {} stopping.", i);
}
