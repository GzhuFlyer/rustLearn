use std::time::{SystemTime, UNIX_EPOCH};
fn main() {
    let sys_time = SystemTime::now();
    // let new_sys_time = SystemTime::now();
    // let difference = new_sys_time
    //     .duration_since(sys_time)
    //     .expect("Clock may have gone backwards");
    // println!("{:?}", difference);

    let duration = sys_time.duration_since(UNIX_EPOCH).unwrap();
    // if let a = Ok((duration.as_secs(), duration.subsec_nanos())) {
    //     println!("a={}", a);
    // }
    // println!(
    //     "Ok((duration.as_secs(), duration.subsec_nanos()))={}",
    //     Ok((duration.as_secs(), duration.subsec_nanos()))
    // );
    println!(
        " duration.as_secs()={},duration.subsec_nanos()={}",
        duration.as_secs(),
        duration.subsec_nanos()
    );
}
