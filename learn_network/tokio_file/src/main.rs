use tokio::fs;
use tokio::fs::File;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt, SeekFrom};
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    // let ret1 = test(0, "hello ".to_string()).await;
    // let ret2 = test(6, "world\n".to_string()).await;
    // println!("ret1 = {:?}, ret2 = {:?}", ret1, ret2);
    println!("Hello, world1!");

    let mut s_in = String::from("");
    let mut i = 0; //128192
    while i < 5 {
        let a = (i % 10).to_string();
        s_in.push_str(&a);
        i += 1;
    }
    // s_in.push('\n');
    println!("s_in.len()={}", s_in.len());
    let ret3 = test(6, s_in).await;
    std::thread::sleep(std::time::Duration::from_millis(100));
}

async fn test(offset: u64, buff: String) -> io::Result<()> {
    let mut file = tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        // .truncate(true)
        .open("/home/fzw/workspace/bky/rustLearn/learn_network/tokio_file/writefile")
        .await?;

    let total = buff.len();
    let mut write_count = 0;
    // let mut loop_offset = offset;

    while write_count < total {
        file.seek(SeekFrom::Start(write_count as u64 + offset))
            .await?;
        // let slice_buf = &buff[loop_offset as usize..total];
        let ret_count = file.write(&buff[write_count..total].as_bytes()).await?;
        write_count += ret_count;
    }

    println!("write_count = {}", write_count);
    println!("Hello, world12");
    Ok(())
}
