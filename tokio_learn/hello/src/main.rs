// #[tokio::main]
// async fn main() {
//     println!("hello");
// }
fn main() {
    tokio_run();
}

fn tokio_run() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        println!("hello");
    })
}
