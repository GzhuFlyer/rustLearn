fn main() {
    println!("Hello, world!");

    let parentStr: &str = "/hello";
    let parentStr_2: &str = "/good/morning/hello/world";

    let childStr;
    let len = parentStr.rfind('/').unwrap() == 0;
    println!("len={}", len);
    if parentStr.rfind('/').unwrap() == 0 {
        childStr = parentStr.as_bytes();
    } else {
        childStr = &parentStr.as_bytes()[0..parentStr.rfind('/').unwrap()];
    }

    println!("parentStr.rfind('/') = {}", parentStr.rfind('/').unwrap());

    let childStr_2 = &parentStr_2.as_bytes()[0..parentStr_2.rfind('/').unwrap()];

    println!("parentStr = {}", parentStr);
    println!(
        "childStr = {}",
        std::str::from_utf8(childStr).unwrap().to_string()
    );

    println!("parentStr_2 = {}", parentStr_2);
    println!(
        "childStr_2 = {}",
        std::str::from_utf8(childStr_2).unwrap().to_string()
    );
}
